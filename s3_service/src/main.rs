#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{http::Status, response::NamedFile, Data};
use std::io::Error;

#[get("/<file_name>")]
fn fetch(file_name: String) -> Result<NamedFile, Error> {
    todo!();
}

#[post("/?<path>", data = "<data>")]
fn store(path: String, data: Data) -> Result<Status, Error> {
    todo!();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/fetch", routes![fetch])
        .mount("/store", routes![store])
}
