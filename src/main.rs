#![feature(byte_slice_trim_ascii)]

mod crypto;
mod test;

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
