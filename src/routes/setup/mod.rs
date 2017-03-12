use std::collections::HashMap;

use rocket::Rocket;
use rocket_contrib::Template;

#[get("/")]
fn setup() -> Template {
    let empty: HashMap<String, String> = HashMap::new();
    Template::render("setup/index", &empty)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/setup", routes![setup])
}