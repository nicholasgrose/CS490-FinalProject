#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Deserialize;

#[get("/<file_name>")]
fn get_file(file_name: String) -> String {
    format!("File: {}!", file_name)
}

#[derive(Deserialize)]
struct UploadData {
    path: String,
    name: String,
    compress: bool,
}

#[post("/", data = "<data>")]
fn upload(data: Json<UploadData>) -> String {
    format!(
        "Path: {}, Name: {}, Compress: {}",
        data.path, data.name, data.compress
    )
}

fn main() {
    rocket::ignite()
        .mount("/getfile", routes![get_file])
        .mount("/upload", routes![upload])
        .launch();
}
