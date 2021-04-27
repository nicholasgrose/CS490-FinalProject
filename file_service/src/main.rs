#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::Data;
use std::io::Error;
use std::io::Read;
use std::fs::File;
use std::io;

const UPLOAD_BYTE_LIMIT: u64 = 250_000_000; // Equivalent to 250 MB

#[get("/<file_name>")]
fn get_file(file_name: String) -> Result<NamedFile, Error> {
    // TODO: Detect and handle decompression if compressed
    NamedFile::open(file_name)
}

#[post("/?<path>&<name>&compress", data = "<data>")]
fn upload_compressed(path: String, name: String, data: Data) -> Result<Status, Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let file_path = format!("{}/{}", path, name);
    let mut file = File::create(&file_path)?;

    // TODO: Handle compression
    io::copy(&mut limited_data, &mut file).map(|num| num.to_string())?;
    Ok(Status::Ok)
}

#[post("/?<path>&<name>", data = "<data>")]
fn upload_uncompressed(path: String, name: String, data: Data) -> Result<Status, Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let file_path = format!("{}/{}", path, name);
    let mut file = File::create(&file_path)?;

    io::copy(&mut limited_data, &mut file)?;
    Ok(Status::Ok)
}

fn main() {
    rocket::ignite()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload_compressed, upload_uncompressed])
        .launch();
}
