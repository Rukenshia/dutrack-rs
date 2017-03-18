use super::rocket::Outcome;
use super::rocket::request::{self, FromRequest, Request};

use super::session::SessionManager;
use super::db::Database;
use super::util;

use db::models::*;
use db::schema::users::dsl::*;
use diesel::prelude::*;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let session_manager = match util::get_state::<SessionManager>(request) {
          Outcome::Success(s) => s,
          _ => return Outcome::Forward(())
        };
        let db = match util::get_state::<Database>(request) {
          Outcome::Success(s) => s,
          _ => return Outcome::Forward(())
        };

        let session_token = request.cookies()
            .find("session_token")
            .and_then(|cookie| cookie.value().parse().ok())
            .unwrap();

        if !session_manager.exists(&session_token) {
          return Outcome::Forward(());
        }

        let uid = match session_manager.get_user(&session_token) {
          Ok(uid) => uid,
          Err(_) => return Outcome::Forward(()),
        };

        let con = db.pg.lock().unwrap();
        match users.filter(id.eq(&uid)).first::<User>(&*con) {
          Ok(u) => Outcome::Success(u),
          Err(_) => Outcome::Forward(()),
        }
    }
}