use rocket::Outcome;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};

use session::SessionManager;
use db::Database;
use util;

use log;

use db::models::*;
use db::schema::users::dsl::*;
use diesel::prelude::*;
use diesel;

use bcrypt::{DEFAULT_COST, hash, verify, BcryptResult};

pub enum RegistrationError {
    EmailInUse,
    EmailFormat,
    PasswordTooShort,
    InternalServerError(String),
}

impl User {
    pub fn hash_password(pw: &str) -> BcryptResult<String> {
        hash(pw, DEFAULT_COST)
    }

    pub fn register(reg_email: &str, reg_password: &str) -> Result<User, RegistrationError> {
        if reg_email.len() < 5 || !reg_email.contains("@") || !reg_email.contains(".") {
            return Err(RegistrationError::EmailFormat);
        }

        if reg_password.len() < 6 {
            return Err(RegistrationError::PasswordTooShort);
        }

        use db::schema::users;

        let con = Database::get().pg.lock().unwrap();
        match users.filter(email.eq(&reg_email)).first::<User>(&*con) {
            Ok(_) => return Err(RegistrationError::EmailInUse),
            Err(_) => (),
        };

        let hashed = match User::hash_password(&reg_password) {
            Ok(p) => p,
            Err(_) => return Err(RegistrationError::InternalServerError("crypto_hash".into())),
        };


        let new_user = NewUser {
            email: &reg_email,
            password: &hashed,
        };


        match diesel::insert(&new_user).into(users::table).get_result::<User>(&*con) {
            Ok(u) => Ok(u),
            Err(_) => Err(RegistrationError::InternalServerError("db_error".into())),
        }
    }

    pub fn verify_password(&self, pw: &str) -> BcryptResult<bool> {
        verify(pw, &self.password)
    }

    pub fn login(&self, cookies: &Cookies) {
        let session_token = SessionManager::get().start(&self.id).unwrap();
        cookies.add(Cookie::new("session_token", session_token));
    }

    pub fn logout(&self, cookies: &Cookies) -> Result<(), String> {
        let session_token = match cookies.find("session_token").and_then(|cookie| {
                                                         Some(cookie.value().to_string())
                                                     }) {
            Some(t) => t,
            None => {
                error!(log::get(),
                       "trying to log out user with non-existing cookie");
                return Ok(());
            }
        };
        cookies.remove("session_token");

        SessionManager::get().end(&session_token).map_err(|e| format!("redis: {}", e))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let session_manager = match util::get_state::<SessionManager>(request) {
            Outcome::Success(s) => s,
            _ => return Outcome::Forward(()),
        };
        let db = match util::get_state::<Database>(request) {
            Outcome::Success(s) => s,
            _ => return Outcome::Forward(()),
        };

        let session_token = match request.cookies().find("session_token").and_then(|cookie| {
                                                                   cookie.value().parse().ok()
                                                               }) {
            Some(v) => v,
            None => return Outcome::Forward(()),
        };

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
