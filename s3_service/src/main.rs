#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate s3;

use rocket::{http::Status, response::Responder, tokio::fs::File};
use s3::{creds::Credentials, Bucket, Region};
use std::{fs, io::{self, Cursor}};

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

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error(error)
    }
}

const BUCKET_NAME: &str = "cs490-final-project-nrose";

fn get_bucket() -> Result<Bucket, anyhow::Error> {
    println!("fail1");
    let region: Region = "us-east-1".parse()?;
    println!("fail2");
    let credentials: Credentials = Credentials::anonymous()?;
    Bucket::new(BUCKET_NAME, region, credentials)
}

#[get("/<file_name>")]
async fn fetch(file_name: String) -> Result<Status, Error> {
    let bucket = get_bucket()?;

    println!("bucket");

    let result = bucket.get_object(&file_name).await;
    println!("passed {}", if let Ok(a) = &result {a.1} else {0});
    if let Err(e) = &result {
        println!("{}", e);
    }
    let (data, code) = result?;

    if code != 200 {
        return Err(Error::from(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not fetch from S3.",
        )));
    }

    let mut file = fs::File::create(&file_name)?;
    io::copy(&mut Cursor::new(data), &mut file)?;

    Ok(Status::Ok)
}

#[get("/<file_name>")]
async fn store(file_name: String) -> Result<Status, Error> {
    let bucket = get_bucket()?;
    println!("fail3");

    let mut file = File::open(&file_name).await?;
    println!("fail4");

    let code = bucket.put_object_stream(&mut file, format!("/{}", &file_name)).await;
    if let Err(e) = &code {
        println!("{}", e);
    }
    let code = code?;

    println!("fail5");

    if code != 200 {
        return Err(Error::from(io::Error::new(
            io::ErrorKind::Interrupted,
            "Could not store in S3.",
        )));
    }

    Ok(Status::Ok)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/fetch", routes![fetch])
        .mount("/store", routes![store])
}
