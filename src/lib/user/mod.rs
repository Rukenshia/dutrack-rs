use super::rocket::Outcome;
use super::rocket::request::{self, FromRequest, Request};

use super::session::Session;

#[allow(dead_code)]
pub struct User {
  session: Session,
}

#[allow(dead_code)]
impl User {
  pub fn new(session: Session) -> Self {
    User { 
      session: session,
    }
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let user = request.cookies()
            .find("session_token")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|token| User::new(token));

        match user {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(())
        }
    }
}