#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate regex;
extern crate reqwest;

use reqwest::StatusCode;
use rocket::{
    data::{Data, ToByteUnit},
    http::{Header, Status},
    response::{
        NamedFile, Responder,
    },
    Request, Response,
};
use std::{
    fs::File,
    io,
    path::Path,
};

pub struct Error(anyhow::Error);

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(
        self,
        request: &'r rocket::Request<'_>,
    ) -> std::result::Result<rocket::Response<'static>, rocket::http::Status> {
        Responder::respond_to(Status::NotFound, request)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error(anyhow::Error::new(error))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error(anyhow::Error::new(error))
    }
}

struct RenameFile<R> {
    responder: R,
    file_name: String,
}

impl<'r, R: Responder<'r, 'static>> Responder<'r, 'static> for RenameFile<R> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut build = Response::build();

        build.merge(self.responder.respond_to(request)?);

        let new_header = Header::new(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", self.file_name),
        );
        build.header_adjoin(new_header).ok()
    }
}

#[get("/<file>")]
async fn get_file(file: String) -> Result<RenameFile<NamedFile>, Error> {
    let client = reqwest::Client::new();
    let compressed_file = format!("{}.gz", &file);
    // File::open(&file)?
    // AsyncStream::new(rx, generator)

    if file_exists(&file) {
        Ok(RenameFile {
            responder: NamedFile::open(&file).await?,
            file_name: file,
        })
    } else if file_exists(&compressed_file) {
        let response = client
            .get(format!("localhost:15707/inflate/{}", &compressed_file))
            .send()
            .await?;
        let mut inflated_file = &mut File::open(&file)?;
        io::copy(&mut response.text().await?.as_bytes(), &mut inflated_file)?;
        Ok(RenameFile {
            responder: NamedFile::open(&file).await?,
            file_name: file,
        })
    } else {
        let s3_response = client
            .get(format!("localhost:62831/fetch/{}", &file))
            .send()
            .await?;

        if s3_response.status() == StatusCode::OK {
            return Ok(RenameFile {
                responder: NamedFile::open(&file).await?,
                file_name: file,
            });
        }

        let s3_response = client
            .get(format!("localhost:62831/fetch/{}", &compressed_file))
            .send()
            .await?;

        if s3_response.status() != StatusCode::OK {
            return Err(Error::from(io::Error::new(
                io::ErrorKind::NotFound,
                "No such file.",
            )));
        }

        let response = client
            .get(format!("localhost:15707/inflate/{}", &compressed_file))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            return Err(Error::from(io::Error::new(
                io::ErrorKind::NotFound,
                "Inflation not possible.",
            )));
        }

        Ok(RenameFile {
            responder: NamedFile::open(&compressed_file).await?,
            file_name: file,
        })
    }
}

fn file_exists(path: &String) -> bool {
    Path::new(path).exists()
}

#[post("/?<path>&compress", data = "<data>")]
async fn upload_compressed(path: String, data: Data) -> Result<Status, Error> {
    data.open(250.mebibytes()).into_file(&path).await?;

    let file: String = name_from_path(&path);
    let client = reqwest::Client::new();

    client
        .get(format!("localhost:15707/compress/{}", &file))
        .send()
        .await?;
    let compressed_file = format!("{}.gz", &file);

    client
        .post(format!("localhost:62831/store/{}", compressed_file))
        .send()
        .await?;

    Ok(Status::Ok)
}

fn name_from_path(path: &String) -> String {
    let path_end = path.rfind("/").unwrap_or(0);
    String::from(path.split_at(path_end).1)
}

#[post("/?<path>", data = "<data>")]
async fn upload_uncompressed(path: String, data: Data) -> Result<Status, Error> {
    data.open(250.mebibytes()).into_file(&path).await?;

    let client = reqwest::Client::new();

    let file = name_from_path(&path);

    client
        .post(format!("localhost:62831/store/{}", file))
        .send()
        .await?;

    Ok(Status::Ok)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload_compressed, upload_uncompressed])
}
