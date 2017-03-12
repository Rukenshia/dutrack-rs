#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod routes;

fn main() {
    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found])
        .launch();
}
