use dutrack_lib::session::SessionManager;
use dutrack_lib::db::models::User;

use rocket::http::{Cookie, Cookies};

pub struct UserController {}

impl UserController {
    pub fn begin_session(user: &User, cookies: &Cookies) {
        let session_token = SessionManager::get().start(&user.id).unwrap();
        cookies.add(Cookie::new("session_token", session_token));
    }
}
