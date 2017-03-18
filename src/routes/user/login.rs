use dutrack_lib::session::SessionManager;
use dutrack_lib::db::Database;
use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FromForm, FlashMessage};
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
pub fn login(sm: State<SessionManager>, flash: Option<FlashMessage>) -> Template {
    let mut data = HashMap::<String, String>::new();
    if let Some(ref m) = flash {
        data.insert("flash".into(), m.msg().into());
    }
    Template::render("user/login/index", &data)
}

#[get("/logout")]
pub fn logout(cookies: &Cookies,
                  user: Option<User>,
                  db: State<Database>,
                  sm: State<SessionManager>)
                  -> Redirect {
                      println!("{:?}", cookies);
    if let None = user {
        return Redirect::to("/");
    }
    
    let session_token = cookies
        .find("session_token")
        .and_then(|cookie| Some(cookie.value().to_string()))
        .unwrap();
    cookies.remove("session_token");
    sm.end(&session_token);

    Redirect::to("/")
}

#[post("/login", data = "<login_data>")]
pub fn post_login(login_data: Form<LoginRequest>,
                  cookies: &Cookies,
                  db: State<Database>,
                  sm: State<SessionManager>)
                  -> Flash<Redirect> {
    use dutrack_lib::db::schema::users::dsl::*;

    let data = login_data.get();

    let hashed = match User::hash_password(&data.password) {
        Ok(p) => p,
        Err(_) => {
            let mut tpl_data = HashMap::<String, String>::new();
            return Flash::error(Redirect::to("/500"), "crypto_hash")
        }
    };

    let con = db.pg.lock().unwrap();
    let user = match users.filter(email.eq(&data.email))
              .first::<User>(&*con) {
        Ok(u) => u,
        Err(_) => {
            return Flash::error(Redirect::to("/login"), "invalid")
        }
    };

    match user.verify_password(&data.password) {
        Ok(true) => {
            let session_token = sm.start(&user.id).unwrap();
                cookies.add(Cookie::new("session_token", session_token));
                Flash::success(Redirect::to("/"), "")
        },
        Ok(false) => {
            return Flash::error(Redirect::to("/login"), "invalid")
        },
        Err(_) => Flash::error(Redirect::to("/500"), "crypto_verify"),
    }
}
