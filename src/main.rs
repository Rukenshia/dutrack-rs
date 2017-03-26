#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate lib;

extern crate chrono;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate uuid;

extern crate diesel;
extern crate dotenv;
use dotenv::dotenv;


mod routes;
mod user;
mod form_models;

fn main() {
    dotenv().ok();

    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found, routes::internal_server_error])
        .manage(lib::session::SessionManager::get())
        .manage(lib::db::Database::get())
        .launch();
}
