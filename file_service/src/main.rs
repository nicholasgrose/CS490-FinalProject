#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::Data;
use regex::Regex;
use std::io::Error;
use std::io::Read;
use std::fs::File;
use std::io;

const UPLOAD_BYTE_LIMIT: u64 = 250_000_000; // Equivalent to 250 MB

#[get("/<file_name>")]
fn get_file(file_name: String) -> Result<NamedFile, Error> {
    let gz_regex = Regex::new("^.+\\.gz$").unwrap();
    let gz_result = gz_regex.find(&file_name);

    if let Option::None = gz_result { // Is not compressed
        NamedFile::open(file_name)
    }
    else { // Is compressed
        // TODO: Handle inflation
        todo!();
    }
}

#[post("/?<path>&compress", data = "<data>")]
fn upload_compressed(path: String, data: Data) -> Result<Status, Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let mut file = File::create(&path)?;

    // TODO: Compress file for saving

    io::copy(&mut limited_data, &mut file).map(|num| num.to_string())?;
    Ok(Status::Ok)
}

#[post("/?<path>", data = "<data>")]
fn upload_uncompressed(path: String, data: Data) -> Result<Status, Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let mut file = File::create(&path)?;

    io::copy(&mut limited_data, &mut file)?;
    Ok(Status::Ok)
}

fn main() {
    rocket::ignite()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload_compressed, upload_uncompressed])
        .launch();
}
