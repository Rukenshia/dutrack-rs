use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FlashMessage};
use rocket::http::Cookies;
use rocket_contrib::Template;
use std::collections::HashMap;

use dutrack_lib::db::models::User;
use form_models::user::LoginRequest;
use dutrack_lib::user::LoginError;

use user::UserController;

#[get("/login")]
#[allow(unused)]
pub fn login_redirect(user: User) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
pub fn login(flash: Option<FlashMessage>) -> Template {
    let mut data = HashMap::<String, String>::new();
    if let Some(ref m) = flash {
        data.insert("flash".into(), m.msg().into());
    }
    Template::render("user/login/index", &data)
}

#[get("/logout")]
pub fn logout(cookies: &Cookies, user: Option<User>) -> Flash<Redirect> {
    if let None = user {
        return Flash::success(Redirect::to("/"), "");
    }

    if let Err(_) = user.unwrap().logout(cookies) {
        return Flash::error(Redirect::to("/500"), "logout_redis_failure");
    };

    Flash::success(Redirect::to("/"), "")
}

#[post("/login", data = "<login_data>")]
pub fn post_login(login_data: Form<LoginRequest>, cookies: &Cookies) -> Flash<Redirect> {
    let data = login_data.get();

    match User::try_login(&data.email, &data.password) {
        Ok(u) => {
            UserController::begin_session(&u, cookies);
            Flash::success(Redirect::to("/"), "")
        }
        Err(e) => {
            match e {
                LoginError::InvalidCredentials => {
                    Flash::error(Redirect::to("/login"), "Invalid email or password.")
                }
                LoginError::InternalServerError(ref e) => Flash::error(Redirect::to("/500"), e),
            }
        }
    }
}
