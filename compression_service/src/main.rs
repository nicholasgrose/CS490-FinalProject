#[macro_use]
extern crate rocket;
extern crate flate2;
extern crate regex;

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use regex::Regex;
use rocket::{
    http::{Header, Status},
    response::{NamedFile, Responder, Response},
    Request,
};
use std::{
    fs::File,
    io::{self, Error},
};

const COMPRESSION_LEVEL: Compression = Compression::new(2);

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

// #[get("/<file_name>")]
// fn compress(file_name: String) -> Result<RenameFile<NamedFile>, Error> {
//     let mut uncompressed_file = File::open(&file_name)?;

//     let compressed_file_name = format!("{}.gz", &file_name);
//     let compressed_file = File::create(&compressed_file_name)?;

//     let mut encoder = GzEncoder::new(&compressed_file, COMPRESSION_LEVEL);

//     io::copy(&mut uncompressed_file, &mut encoder)?;

//     Ok(RenameFile {
//         responder: NamedFile::open(&compressed_file_name)?,
//         file_name: compressed_file_name,
//     })
// }

#[get("/<file_name>")]
fn compress(file_name: String) -> Result<Status, Error> {
    let mut uncompressed_file = File::open(&file_name)?;

    let compressed_file_name = format!("{}.gz", &file_name);
    let compressed_file = File::create(&compressed_file_name)?;

    let mut encoder = GzEncoder::new(&compressed_file, COMPRESSION_LEVEL);

    io::copy(&mut uncompressed_file, &mut encoder)?;

    Ok(Status::Ok)
}

#[get("/<file_name>")]
async fn inflate(file_name: String) -> Result<RenameFile<NamedFile>, Error> {
    let compressed_file = File::open(&file_name)?;
    let mut decoder = GzDecoder::new(&compressed_file);

    let gz_regex = Regex::new("^.+\\.gz$").unwrap();
    let gz_result = gz_regex.find(&file_name);

    if let Option::None = gz_result {
        return Err(Error::from(io::ErrorKind::InvalidData));
    }

    let inflated_file_name = String::from(file_name.split_at(file_name.len() - 3).0);
    let mut inflated_file = File::create(&inflated_file_name)?;

    io::copy(&mut decoder, &mut inflated_file)?;

    Ok(RenameFile {
        responder: NamedFile::open(&inflated_file_name).await?,
        file_name: inflated_file_name,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/compress", routes![compress])
        .mount("/inflate", routes![inflate])
}
