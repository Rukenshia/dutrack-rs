use std::collections::HashMap;

use rocket::Rocket;
use rocket::request::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;

mod assets;
mod setup;
mod user;

use dutrack_lib::user::User;

#[get("/")]
fn index_user(user: User) -> Template {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("user_id".into(), user.id);

    Template::render("index", &data)
}

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

pub fn mount(rocket: Rocket) -> Rocket {
    let mut r = rocket.mount("/", routes![index, index_user]);
    r = assets::mount(r);
    r = user::mount(r);
    setup::mount(r)
}