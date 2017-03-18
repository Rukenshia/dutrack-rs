use dutrack_lib::session::SessionManager;
use dutrack_lib::db::Database;
use dutrack_lib::db::models::User;
use rocket::response::Redirect;
use rocket::request::FromForm;
use rocket::{Rocket, State};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;

use std::collections::HashMap;

mod login;
mod register;

use self::login::*;
use self::register::*;

#[get("/")]
pub fn index(user: User) -> Template {
    let mut data: HashMap<String, String> = HashMap::new();

    Template::render("index", &data)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![login, logout, post_login, register, post_register, login_redirect, register_redirect])
}
