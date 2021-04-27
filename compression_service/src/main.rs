#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use rocket::response::NamedFile;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::http::Header;
use rocket::http::Status;
use rocket::Request;
use regex::Regex;
use std::io::Error;
use std::fs::File;
use std::io;

const COMPRESSION_LEVEL: Compression = Compression::new(2);

struct RenameFile<R> {
    responder: R,
    file_name: String,
}

impl<'r, R: Responder<'r>> Responder<'r> for RenameFile<R> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();

        build.merge(self.responder.respond_to(req)?);

        let new_header = Header::new(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", self.file_name),
        );
        build.header_adjoin(new_header).ok()
    }
}

#[get("/<filename>")]
fn compress(filename: String) -> Result<RenameFile<NamedFile>, Error> {
    let mut uncompressed_file = File::open(&filename)?;

    let compressed_file_name = format!("{}.gz", &filename);
    let compressed_file = File::create(&compressed_file_name)?;

    let mut encoder = GzEncoder::new(&compressed_file, COMPRESSION_LEVEL);

    io::copy(&mut uncompressed_file, &mut encoder)?;

    Ok(RenameFile {
        responder: NamedFile::open(&compressed_file_name)?,
        file_name: compressed_file_name,
    })
}

#[get("/<filename>")]
fn inflate(filename: String) -> Result<RenameFile<NamedFile>, Error> {
    let compressed_file = File::open(&filename)?;
    let mut decoder = GzDecoder::new(&compressed_file);

    let gz_regex = Regex::new("^.+\\.gz$").unwrap();
    let gz_result = gz_regex.find(&filename);

    if let Option::None = gz_result {
        return Err(Error::from(io::ErrorKind::InvalidData));
    }

    let inflated_file_name = String::from(filename.split_at(filename.len() - 3).0);
    let mut inflated_file = File::create(&inflated_file_name)?;

    io::copy(&mut decoder, &mut inflated_file)?;

    Ok(RenameFile {
        responder: NamedFile::open(&inflated_file_name)?,
        file_name: inflated_file_name,
    })
}

fn main() {
    rocket::ignite()
        .mount("/compress", routes![compress])
        .mount("/inflate", routes![inflate])
        .launch();
}
