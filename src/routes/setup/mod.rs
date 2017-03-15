use std::collections::HashMap;

use rocket::Rocket;
use rocket_contrib::Template;
use lib::key::Key;

#[get("/")]
fn setup() -> Template {
    let personal_key = Key::new();

    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("enter_uri".into(), format!("http://localhost:8000/api/v1/fence/{}/enter", personal_key));
    data.insert("exit_uri".into(), format!("http://localhost:8000/api/v1/fence/{}/exit", personal_key));
    Template::render("setup/index", &data)
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/setup", routes![setup])
}