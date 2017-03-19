use rocket::response::{Redirect, Flash};
use rocket::request::{Form, FlashMessage};
use rocket::http::Cookies;
use rocket_contrib::Template;
use std::collections::HashMap;

use dutrack_lib::db::models::User;
use dutrack_lib::user::RegistrationError;
use form_models::user::RegistrationRequest;

use user::UserController;

#[get("/register")]
#[allow(unused)]
pub fn register_redirect(user: User) -> Redirect {
    Redirect::to("/")
}

#[get("/register")]
pub fn register(flash: Option<FlashMessage>) -> Template {
    let mut data = HashMap::<String, String>::new();
    if let Some(ref m) = flash {
        data.insert("flash".into(), m.msg().to_owned());
    }
    println!("{:?}", data);
    Template::render("user/login/register", &data)
}

#[post("/register", data = "<register_data>")]
pub fn post_register(register_data: Form<RegistrationRequest>,
                     cookies: &Cookies)
                     -> Flash<Redirect> {
    let data = register_data.get();

    match User::register(&data.email, &data.password) {
        Ok(u) => {
            UserController::begin_session(&u, cookies);
            Flash::success(Redirect::to("/"), "")
        }
        Err(res) => {
            match res {
                RegistrationError::EmailInUse => {
                    Flash::error(Redirect::to("/register"), "This email address is in use.")
                }
                RegistrationError::EmailFormat => {
                    Flash::error(Redirect::to("/register"),
                                 "Please enter a valid Email address.")
                }
                RegistrationError::PasswordTooShort => {
                    Flash::error(Redirect::to("/register"),
                                 "The chosen password is too short.")
                }
                RegistrationError::InternalServerError(e) => Flash::error(Redirect::to("/500"), e),
            }
        }
    }
}
