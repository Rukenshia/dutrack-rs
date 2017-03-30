use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};

use session::SessionManager;
use db::Database;

use log::LOGGER;

use db::models::*;
use diesel::prelude::*;
use diesel;

use uuid::Uuid;

use bcrypt::{DEFAULT_COST, hash, verify, BcryptResult};

pub enum RegistrationError {
    EmailInUse,
    EmailFormat,
    PasswordTooShort,
    InternalServerError(String),
}

pub enum LoginError {
    InvalidCredentials,
    InternalServerError(String),
}

impl User {
    pub fn hash_password(pw: &str) -> BcryptResult<String> {
        hash(pw, DEFAULT_COST)
    }

    pub fn from_fence(fence_id: &Uuid) -> Result<User, String> {
        use db::schema::users::dsl::*;

        let con = Database::get().pg.lock().unwrap();
        match users.filter(fence_key.eq(fence_id)).first::<User>(&*con) {
            Ok(u) => Ok(u),
            Err(e) => Err(format!("db: {}", e)),
        }
    }

    pub fn register(reg_email: &str, reg_password: &str) -> Result<User, RegistrationError> {
        use db::schema::users::dsl::*;

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

        debug!(LOGGER, "creating user with email {}", reg_email);
        match diesel::insert(&new_user).into(users::table).get_result::<User>(&*con) {
            Ok(u) => Ok(u),
            Err(_) => Err(RegistrationError::InternalServerError("db_error".into())),
        }
    }

    pub fn try_login(login_email: &str, login_password: &str) -> Result<User, LoginError> {
        use db::schema::users::dsl::*;

        let con = Database::get().pg.lock().unwrap();
        let user = match users.filter(email.eq(login_email)).first::<User>(&*con) {
            Ok(u) => u,
            Err(_) => return Err(LoginError::InvalidCredentials),
        };

        match user.verify_password(login_password) {
            Ok(true) => Ok(user),
            Ok(false) => return Err(LoginError::InvalidCredentials),
            Err(_) => Err(LoginError::InternalServerError("crypto_verify".into())),
        }
    }

    pub fn verify_password(&self, pw: &str) -> BcryptResult<bool> {
        verify(pw, &self.password)
    }

    pub fn get_stamps(&self) -> Result<Vec<Stamp>, String> {
        use db::schema::stamps::dsl::*;

        let con = Database::get().pg.lock().unwrap();

        match stamps.filter(fence.eq(self.fence_key)).load::<Stamp>(&*con) {
            Ok(s) => Ok(s),
            Err(e) => return Err(format!("db: {}", e)),
        }
    }

    pub fn finish_setup(&mut self) -> Result<(), String> {
        use db::schema::users::dsl::*;

        if self.finished_setup {
            return Ok(());
        }

        self.finished_setup = true;

        let con = Database::get().pg.lock().unwrap();

        match diesel::update(users.find(&self.id))
                  .set(finished_setup.eq(true))
                  .returning(id)
                  .get_result::<Uuid>(&*con) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("db {}", e)),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        use db::schema::users::dsl::*;
        let session_manager = SessionManager::get();
        let db = Database::get();

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
