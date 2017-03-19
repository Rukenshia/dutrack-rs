use slog;
use slog_term;
use slog::DrainExt;

lazy_static! {
    pub static ref LOGGER: slog::Logger = {
        let drain = slog_term::streamer().build().fuse();
        slog::Logger::root(drain, o!())
    };
}

pub fn debug(msg: &str) {
    debug!(LOGGER, "{}", msg);
}

pub fn info(msg: &str) {
    info!(LOGGER, "{}", msg);
}

pub fn warn(msg: &str) {
    warn!(LOGGER, "{}", msg);
}

pub fn error(msg: &str) {
    error!(LOGGER, "{}", msg);
}

pub fn get() -> slog::Logger {
    LOGGER.clone()
}

pub fn new(o: Option<Box<slog::ser::SyncMultiSerialize + 'static>>) -> slog::Logger {
    LOGGER.new(o)
}
