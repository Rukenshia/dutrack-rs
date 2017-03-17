#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dutrack_lib;

mod routes;

fn main() {
    let session_manager = dutrack_lib::init();

    routes::mount(rocket::ignite())
        .catch(errors![routes::not_found])
        .manage(session_manager)
        .launch();
}
