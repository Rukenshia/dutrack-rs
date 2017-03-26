#![feature(use_extern_macros)]

extern crate uuid;
extern crate redis;
extern crate rocket;

extern crate chrono;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

#[macro_use]
extern crate slog;
extern crate slog_term;

extern crate bcrypt;

pub mod util;
pub mod session;
pub mod user;
pub mod stamp;
pub mod log;
pub mod db;
pub mod workday;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::init();
    }
}
