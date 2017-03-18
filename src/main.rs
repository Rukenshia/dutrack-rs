#![feature(plugin, custom_derive, custom_attribute, use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dutrack_lib;

extern crate uuid;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate dotenv;
use dotenv::dotenv;
use std::env;

mod routes;

fn main() {
    dotenv().ok();

    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found, routes::internal_server_error])
        .manage(dutrack_lib::session::SessionManager::get())
        .manage(dutrack_lib::db::Database::get())
        .launch();
}
