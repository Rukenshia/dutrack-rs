#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod routes;
use routes::util;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

fn main() {
    rocket::ignite().mount("/", routes![index, util::favicon::get]).launch();
}
