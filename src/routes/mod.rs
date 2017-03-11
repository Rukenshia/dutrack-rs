use std::collections::HashMap;

use rocket::request::Request;
use rocket_contrib::Template;

pub mod assets;

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}