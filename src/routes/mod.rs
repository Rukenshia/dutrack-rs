use std::collections::HashMap;

use rocket::Rocket;
use rocket::request::{Request, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket::http::uri::URI;
use rocket_contrib::Template;

mod assets;
mod setup;
mod user;

use dutrack_lib::db::models::User;

#[get("/", rank = 2)]
fn index() -> Redirect {
    Redirect::to("/login")
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

#[get("/500")]
pub fn display_500(uri: &URI, flash: Option<FlashMessage>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", uri.as_str());
    if let Some(ref m) = flash {
        map.insert("int_msg".into(), m.msg().into());
    }
    Template::render("error/500", &map)
}

pub fn mount(rocket: Rocket) -> Rocket {
    let mut r = rocket.mount("/", routes![index, user::index, display_500]);
    r = assets::mount(r);
    r = user::mount(r);
    setup::mount(r)
}
