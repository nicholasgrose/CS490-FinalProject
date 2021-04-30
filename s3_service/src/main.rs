#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{http::Status, response::NamedFile, Data};
use std::io::Error;

const UPLOAD_BYTE_LIMIT: u64 = 250_000_000; // Equivalent to 250 MB

#[get("/<file_name>")]
fn fetch(file_name: String) -> Result<NamedFile, Error> {
    todo!();
}

#[post("/?<path>", data = "<data>")]
fn store(path: String, data: Data) -> Result<Status, Error> {
    todo!();
}

fn main() {
    rocket::ignite()
        .mount("/fetch", routes![fetch])
        .mount("/store", routes![store])
        .launch();
}
