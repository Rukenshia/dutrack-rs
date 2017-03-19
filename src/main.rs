#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dutrack_lib;

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
        .manage(dutrack_lib::session::SessionManager::get())
        .manage(dutrack_lib::db::Database::get())
        .launch();
}
