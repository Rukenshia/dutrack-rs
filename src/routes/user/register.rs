use dutrack_lib::session::SessionManager;
use dutrack_lib::db::Database;
use rocket::response::{Redirect, Flash};
use rocket::request::{Form, FromForm, FlashMessage};
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
pub fn register(sm: State<SessionManager>, flash: Option<FlashMessage>) -> Template {
    let mut data = HashMap::<String, String>::new();
    if let Some(ref m) = flash {
        data.insert("flash".into(), m.msg().into());
    }
    Template::render("user/login/register", &data)
}

#[post("/register", data = "<register_data>")]
pub fn post_register(register_data: Form<RegisterRequest>,
                  cookies: &Cookies,
                  db: State<Database>,
                  sm: State<SessionManager>)
                  -> Flash<Redirect> {
    use dutrack_lib::db::schema::users::dsl::*;
    use dutrack_lib::db::schema::users;

    let data = register_data.get();

    let con = db.pg.lock().unwrap();
    match users.filter(email.eq(&data.email))
              .first::<User>(&*con) {
        Ok(_) => return Flash::error(Redirect::to("/register"), "This email is already registered."),
        Err(_) => (),
    };

    let hashed = match User::hash_password(&data.password) {
        Ok(p) => p,
        Err(_) => return Flash::error(Redirect::to("/500"), "crypto_hash")
    };


    let new_user = NewUser {
      email: &data.email,
      password: &hashed,
    };


    match diesel::insert(&new_user).into(users::table)
        .get_result(&*con) {
          Ok(u) => {
            let session_token = sm.start(&(&u as &User).id).unwrap();
            cookies.add(Cookie::new("session_token", session_token));
            Flash::success(Redirect::to("/"), "")
          }
          Err(_) => Flash::error(Redirect::to("/500"), "db_insert")
        }
}
