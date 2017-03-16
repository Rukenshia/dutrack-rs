use super::redis;
use super::redis::{Commands, RedisError};
use super::uuid::Uuid;

use std::sync::Mutex;

#[allow(dead_code)]
pub type Session = String;

#[allow(dead_code)]
pub struct SessionManager {
  rds: Mutex<redis::Connection>,
}

#[allow(dead_code)]
impl SessionManager {
  pub fn new(connection: &'static str) -> Self {
    let client = redis::Client::open(connection).unwrap();

    SessionManager {
      rds: Mutex::new(client.get_connection().unwrap()),
    }
  }

  pub fn exists(&self, session: &Session) -> bool {
    let rds = self.rds.lock().unwrap();

    rds.exists::<&Session, ()>(session).is_ok()
  }

  pub fn get_user(&self, session: &Session) -> Result<String, RedisError> {
    let rds = self.rds.lock().unwrap();
    
    rds.get::<&Session, String>(&session)
  }

  pub fn start(&mut self, user: &str) -> Result<Session, RedisError> {
    let rds = self.rds.lock().unwrap();

    let session_key = Uuid::new_v4().to_string();

    if let Err(e) = rds.set::<&str, &str, ()>(&session_key, user) {
      return Err(e)
    }

    // set the expire to 7 days
    match rds.expire::<&Session, ()>(&session_key, 3600 * 24 * 7) {
      Ok(_) => Ok(session_key),
      Err(e) => Err(e)
    }
  }
}