use std::collections::HashMap;

use rocket::Rocket;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use lib::db::models::User;

#[get("/")]
fn setup(user: User) -> Template {
    let mut data: HashMap<String, String> = HashMap::new();
    data.insert("enter_uri".into(),
                format!("http://time.in.fkn.space/api/v1/fence/{}/enter",
                        user.fence_key));
    data.insert("exit_uri".into(),
                format!("http://time.in.fkn.space/api/v1/fence/{}/exit",
                        user.fence_key));
    Template::render("setup/index", &data)
}

#[get("/", rank = 2)]
fn setup_redir() -> Redirect {
    Redirect::to("/")
}

#[get("/finish")]
fn finish_setup(mut user: User) -> Flash<Redirect> {
    match user.finish_setup() {
        Ok(_) => Flash::success(Redirect::to("/"), ""),
        Err(e) => Flash::error(Redirect::to("/500"), e),
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/setup", routes![setup, setup_redir, finish_setup])
}
