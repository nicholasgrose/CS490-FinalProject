#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket::Data;
use std::io::Error;

const UPLOAD_BYTE_LIMIT: u64 = 250_000_000; // Equivalent to 250 MB

#[get("/<file_name>")]
fn get_file(file_name: String) -> Result<NamedFile, Error> {
    todo!();
}

#[post("/?<path>", data = "<data>")]
fn upload(path: String, data: Data) -> Result<Status, Error> {
    todo!();
}

fn main() {
    rocket::ignite()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload])
        .launch();
}
