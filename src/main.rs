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

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let session_manager = dutrack_lib::session::SessionManager::new(&redis_url);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database = dutrack_lib::db::Database::connect(&database_url);

    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found, routes::internal_server_error])
        .manage(session_manager)
        .manage(database)
        .launch();
}
