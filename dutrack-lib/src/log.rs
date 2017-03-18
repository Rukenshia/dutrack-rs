use slog;
use slog_term;
use slog::DrainExt;

lazy_static! {
    static ref LOGGER: slog::Logger = {
        let drain = slog_term::streamer().build().fuse();
        slog::Logger::root(drain, o!())
    };
}

pub fn get() -> slog::Logger {
    LOGGER.clone()
}

pub fn new(o: Option<Box<slog::ser::SyncMultiSerialize + 'static>>) -> slog::Logger {
    LOGGER.new(o)
}
