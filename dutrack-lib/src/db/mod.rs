use diesel::prelude::*;
use diesel::pg::PgConnection;

use slog;
use log;

use std::sync::Mutex;
use uuid::Uuid;

pub mod schema;
pub mod models;

#[allow(dead_code)]
pub struct Database {
    pub pg: Mutex<PgConnection>,
    log: slog::Logger,
}

impl Database {
    pub fn connect(database_url: &str) -> Self {
        let pg = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}",
                                                                        database_url));

        Database {
            pg: Mutex::new(pg),
            log: log::new(o!()),
        }
    }
}
