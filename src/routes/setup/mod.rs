use std::collections::HashMap;

use rocket::Rocket;
use rocket::response::Redirect;
use rocket_contrib::Template;

use dutrack_lib::db::models::User;

#[get("/")]
fn setup(user: User) -> Template {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("enter_uri".into(),
                format!("http://localhost:8000/api/v1/fence/{}/enter",
                        user.fence_key));
    data.insert("exit_uri".into(),
                format!("http://localhost:8000/api/v1/fence/{}/exit", user.fence_key));
    Template::render("setup/index", &data)
}

#[get("/", rank = 2)]
fn setup_redir() -> Redirect {
    Redirect::to("/")
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/setup", routes![setup, setup_redir])
}
