use std::collections::HashMap;

use rocket::Rocket;
use rocket::request::Request;
use rocket_contrib::Template;

mod assets;
mod setup;

#[get("/")]
fn index() -> Template {
    let empty: HashMap<String, String> = HashMap::new();
    Template::render("index", &empty)
}

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

pub fn mount(rocket: Rocket) -> Rocket {
    let mut r = rocket.mount("/", routes![index]);
    r = assets::mount(r);
    setup::mount(r)
}