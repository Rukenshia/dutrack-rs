use std::collections::HashMap;

use rocket::Rocket;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

mod assets;
mod setup;
mod user;

use dutrack_lib::db::models::User;

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/setup")
}

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

#[error(500)]
pub fn internal_server_error(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/500", &map)
}

pub fn mount(rocket: Rocket) -> Rocket {
    let mut r = rocket.mount("/", routes![index, user::index]);
    r = assets::mount(r);
    r = user::mount(r);
    setup::mount(r)
}
