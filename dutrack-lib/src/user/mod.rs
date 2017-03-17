use super::rocket::Outcome;
use super::rocket::request::{self, FromRequest, Request};

use super::session::SessionManager;
use super::util;

#[allow(dead_code)]
pub struct User {
  pub id: String,
}

#[allow(dead_code)]
impl User {
  pub fn new(id: String) -> Self {
    User { 
      id: id,
    }
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let session_manager = match util::get_state::<SessionManager>(request) {
          Outcome::Success(s) => s,
          _ => return Outcome::Forward(())
        };

        let session_token = request.cookies()
            .find("session_token")
            .and_then(|cookie| cookie.value().parse().ok())
            .unwrap();

        match session_manager.exists(&session_token) {
          true => Outcome::Success(User::new(session_manager.get_user(&session_token).unwrap())),
          false => Outcome::Forward(()),
        }
    }
}