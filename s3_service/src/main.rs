#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::Data;
use std::io::Read;

const UPLOAD_BYTE_LIMIT: u64 = 250_000_000; // Equivalent to 250 MB

#[get("/<file_name>")]
fn get_file(file_name: String) -> Result<NamedFile, std::io::Error> {
    // TODO: Handle decompression if compressed
    NamedFile::open(file_name)
}

#[post("/?<path>&<name>&compress", data = "<data>")]
fn upload_compressed(path: String, name: String, data: Data) -> Result<String, std::io::Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let file_path = format!("{}/{}", path, name);
    let mut file = std::fs::File::create(&file_path)?;

    // TODO: Handle compression
    std::io::copy(&mut limited_data, &mut file).map(|num| num.to_string())
}

#[post("/?<path>&<name>", data = "<data>")]
fn upload_uncompressed(path: String, name: String, data: Data) -> Result<String, std::io::Error> {
    let mut limited_data = data.open().take(UPLOAD_BYTE_LIMIT);
    let file_path = format!("{}/{}", path, name);
    let mut file = std::fs::File::create(&file_path)?;

    std::io::copy(&mut limited_data, &mut file).map(|num| num.to_string())
}

fn main() {
    rocket::ignite()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload_compressed, upload_uncompressed])
        .launch();
}
