use dutrack_lib::db::models::User;
use rocket::Rocket;
use rocket::response::Redirect;
use rocket_contrib::Template;

use std::collections::HashMap;

mod login;
mod registration;

use self::login::*;
use self::registration::*;

#[get("/")]
#[allow(unused)]
pub fn index(user: User) -> Result<Template, Redirect> {
    if !user.finished_setup {
        return Err(Redirect::to("/setup"));
    }
    let data: HashMap<String, String> = HashMap::new();
    Ok(Template::render("index", &data))
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/",
                 routes![login,
                         logout,
                         post_login,
                         register,
                         post_register,
                         login_redirect,
                         register_redirect])
}
