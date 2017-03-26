use lib::db::models::User;
use rocket::Rocket;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

mod login;
mod registration;

use self::login::*;
use self::registration::*;

#[derive(Serialize)]
struct IndexContext {
    user: FrontendUser,
}

#[derive(Serialize)]
struct FrontendUser {
    email: String,
    fence: String,
}

impl FrontendUser {
    pub fn from_user(user: &User) -> Self {
        FrontendUser {
            email: user.email.clone(),
            fence: format!("{}", user.fence_key),
        }
    }
}

#[get("/")]
#[allow(unused)]
pub fn index(user: User) -> Result<Template, Flash<Redirect>> {
    if !user.finished_setup {
        return Err(Flash::error(Redirect::to("/setup"), ""));
    }


    let ctx = IndexContext { user: FrontendUser::from_user(&user) };

    Ok(Template::render("index", &ctx))
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
