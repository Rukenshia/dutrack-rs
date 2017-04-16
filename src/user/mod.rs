use lib::session::SessionManager;
use lib::db::models::User;
use lib::log::debug;

use rocket::http::{Cookie, Cookies};


#[derive(Serialize)]
pub struct FrontendUser {
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
