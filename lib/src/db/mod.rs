use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use slog;
use log;

use std::sync::Mutex;

pub mod schema;
pub mod models;

lazy_static! {
    static ref DATABASE: Database = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database::connect(&database_url)
    };
}

#[allow(dead_code)]
pub struct Database {
    pub pg: Mutex<PgConnection>,
    log: slog::Logger,
}

impl Database {
    pub fn get() -> &'static Database {
        &DATABASE as &Database
    }

    pub fn connect(database_url: &str) -> Self {
        let pg = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}",
                                                                        database_url));

        Database {
            pg: Mutex::new(pg),
            log: log::new(o!()),
        }
    }
}
