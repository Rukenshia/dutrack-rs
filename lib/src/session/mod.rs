use redis;
use redis::{Commands, RedisError};
use uuid::Uuid;
use slog;
use log;
use dotenv::dotenv;

use std::env;
use std::sync::Mutex;

#[allow(dead_code)]
pub type Session = String;

#[allow(dead_code)]
pub struct SessionManager {
    rds: Mutex<redis::Connection>,
    log: slog::Logger,
}

lazy_static! {
    static ref SESSION_MANAGER: SessionManager = {
        dotenv().ok();

        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
        SessionManager::new(&redis_url)
    };
}

#[allow(dead_code)]
impl SessionManager {
    pub fn get() -> &'static SessionManager {
        &SESSION_MANAGER as &SessionManager
    }

    pub fn new(connection: &str) -> Self {
        let client = redis::Client::open(connection).unwrap();

        SessionManager {
            log: log::new(o!()),
            rds: Mutex::new(client.get_connection().unwrap()),
        }
    }

    pub fn exists(&self, session: &Session) -> bool {
        let rds = self.rds.lock().unwrap();

        let res = rds.exists::<&Session, ()>(session).is_ok();

        if !res {
            debug!(self.log, "found invalid session {}", session);
        }

        res
    }

    pub fn get_user(&self, session: &Session) -> Result<Uuid, String> {
        let rds = self.rds.lock().unwrap();

        debug!(self.log, "get user for session {}", session);

        let uuid_str = match rds.get::<&Session, String>(&session) {
            Ok(s) => s,
            Err(e) => return Err(format!("redis: {}", e)),
        };

        Uuid::parse_str(&uuid_str).map_err(|e| format!("uuid: {}", e))
    }

    pub fn end(&self, session: &Session) -> Result<(), RedisError> {
        let rds = self.rds.lock().unwrap();

        debug!(self.log, "force-ending session {}", session);
        rds.del::<&Session, ()>(&session)
    }

    pub fn start(&self, user: &Uuid) -> Result<Session, RedisError> {
        let rds = self.rds.lock().unwrap();

        let session_key = Uuid::new_v4().to_string();

        debug!(self.log, "starting session {} for {}", session_key, user);

        if let Err(e) = rds.set::<&str, &str, ()>(&session_key, &user.to_string()) {
            return Err(e);
        }

        // set the expire to 7 days
        match rds.expire::<&Session, ()>(&session_key, 3600 * 24 * 7) {
            Ok(_) => Ok(session_key),
            Err(e) => Err(e),
        }
    }
}
