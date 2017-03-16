#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

mod lib;
mod routes;

fn main() {
    let session_manager = lib::session::SessionManager::new("");

    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found])
        .manage(session_manager)
        .launch();
}
