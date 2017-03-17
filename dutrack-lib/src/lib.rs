extern crate uuid;
extern crate redis;
extern crate rocket;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

pub mod util;
pub mod key;
pub mod session;
pub mod user;

pub fn init() -> session::SessionManager {
    let drain = slog_term::streamer().build().fuse();
    let root_logger = slog::Logger::root(drain, o!());
    info!(root_logger, "Application started");

    session::SessionManager::new(root_logger, "redis://127.0.0.1/")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::init();
    }
}
