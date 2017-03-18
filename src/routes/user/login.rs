use dutrack_lib::session::SessionManager;
use dutrack_lib::db::Database;
use rocket::response::Redirect;
use rocket::request::{Form, FromForm};
use rocket::{Rocket, State};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use std::collections::HashMap;

use diesel::prelude::*;
use dutrack_lib::db::models::User;

#[derive(FromForm)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[get("/login")]
pub fn login_redirect(user: User) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
pub fn login(sm: State<SessionManager>) -> Template {
    Template::render("user/login/index", &HashMap::<String, String>::new())
}

#[post("/login", data = "<login_data>")]
pub fn post_login(login_data: Form<LoginRequest>,
                  cookies: &Cookies,
                  db: State<Database>,
                  sm: State<SessionManager>)
                  -> Result<Redirect, Template> {
    use dutrack_lib::db::schema::users::dsl::*;

    let data = login_data.get();

    let hashed = match User::hash_password(&data.password) {
        Ok(p) => p,
        Err(_) => {
            let mut tpl_data = HashMap::<String, String>::new();
            return Ok(Redirect::to("/500"))
        }
    };

    let con = db.pg.lock().unwrap();
    let user = match users.filter(email.eq(&data.email))
              .first::<User>(&*con) {
        Ok(u) => u,
        Err(e) => {
            let mut tpl_data = HashMap::<String, String>::new();
            tpl_data.insert("has_error".into(), "y".into());
            tpl_data.insert("email".into(), data.email.clone());
            return Err(Template::render("user/login/index", &tpl_data))
        }
    };

    match user.verify_password(&data.password) {
        Ok(true) => {
            let session_token = sm.start(&user.id).unwrap();
                cookies.add(Cookie::new("session_token", session_token));
                Ok(Redirect::to("/"))
        },
        Ok(false) => {
            let mut tpl_data = HashMap::<String, String>::new();
            tpl_data.insert("has_error".into(), "y".into());
            tpl_data.insert("email".into(), data.email.clone());
            return Err(Template::render("user/login/index", &tpl_data))
        },
        Err(_) => Ok(Redirect::to("/500"))
    }
}
