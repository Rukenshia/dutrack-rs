use dutrack_lib::session::SessionManager;
use dutrack_lib::db::Database;
use rocket::response::Redirect;
use rocket::request::{Form, FromForm};
use rocket::{Rocket, State};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::Template;
use std::collections::HashMap;

use diesel;
use diesel::prelude::*;
use dutrack_lib::db::models::{User, NewUser};

use uuid::Uuid;

#[derive(FromForm)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[get("/register")]
pub fn register_redirect(user: User) -> Redirect {
    Redirect::to("/")
}

#[get("/register")]
pub fn register(sm: State<SessionManager>) -> Template {
    let mut data = HashMap::<String, String>::new();
    Template::render("user/login/register", &data)
}

#[post("/register", data = "<register_data>")]
pub fn post_register(register_data: Form<RegisterRequest>,
                  cookies: &Cookies,
                  db: State<Database>,
                  sm: State<SessionManager>)
                  -> Result<Template, String> {
    use dutrack_lib::db::schema::users::dsl::*;
    use dutrack_lib::db::schema::users;

    let data = register_data.get();

    let con = db.pg.lock().unwrap();
    match users.filter(email.eq(&data.email))
              .first::<User>(&*con) {
        Ok(_) => {
          let mut tpl_data = HashMap::<String, String>::new();
          tpl_data.insert("has_error".into(), "user_exists".into());
          return Ok(Template::render("user/login/register", &tpl_data));
        },
        Err(_) => (),
    };

    let hashed = match User::hash_password(&data.password) {
        Ok(p) => p,
        Err(_) => {
            let mut tpl_data = HashMap::<String, String>::new();
            return Err(String::new())
        }
    };


    let new_user = NewUser {
      email: &data.email,
      password: &hashed,
    };


    match diesel::insert(&new_user).into(users::table)
        .get_result(&*con)
        .map_err(|_| "db error".into()) {
          Ok(u) => {
            let session_token = sm.start(&(&u as &User).id).unwrap();
            cookies.add(Cookie::new("session_token", session_token));
            Ok(super::index(u))
          }
          Err(e) => Err(e),
        }
}
