#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod routes;
use routes::assets;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

fn main() {
    assets::mount(rocket::ignite())
        .mount("/", routes![index])
        .catch(errors![routes::not_found])
        .launch();
}
