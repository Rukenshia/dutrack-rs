use dutrack_lib::session::SessionManager;
use dutrack_lib::db::models::User;
use dutrack_lib::log::debug;

use rocket::http::{Cookie, Cookies};

pub struct UserController {}

impl UserController {
    pub fn begin_session(user: &User, cookies: &Cookies) {
        let session_token = SessionManager::get().start(&user.id).unwrap();
        cookies.add(Cookie::new("session_token", session_token));
    }

    pub fn logout(cookies: &Cookies) -> Result<(), String> {
        let session_token = match cookies.find("session_token").and_then(|cookie| {
                                                         Some(cookie.value().to_string())
                                                     }) {
            Some(t) => t,
            None => {
                debug("trying to log out user with non-existing cookie");
                return Ok(());
            }
        };
        debug(&format!("removing session token {}", session_token));
        cookies.remove("session_token");

        SessionManager::get().end(&session_token).map_err(|e| format!("redis: {}", e))
    }
}
