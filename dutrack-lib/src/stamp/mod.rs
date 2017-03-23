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

pub enum FenceEvent {
    Enter,
    Exit,
}

impl FenceEvent {
    pub fn as_str(&self) -> &str {
        match self {
            &FenceEvent::Enter => "enter",
            &FenceEvent::Exit => "exit",
        }
    }
}

impl Stamp {
    pub fn create(fence: &Uuid, ev: FenceEvent) -> Option<Self> {
        None
    }
}