use lib::db::models::User;
use rocket::Rocket;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

mod login;
mod registration;

use self::login::*;
use self::registration::*;
use user::FrontendUser;
use context::Context;

#[get("/")]
#[allow(unused)]
pub fn index(user: User) -> Result<Template, Flash<Redirect>> {
    if !user.finished_setup {
        return Err(Flash::error(Redirect::to("/setup"), ""));
    }

    Ok(Template::render("index", &Context::new(Some(user))))
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
