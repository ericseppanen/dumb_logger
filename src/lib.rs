//! dumb_logger is a minimal logging module.
//!
//! It allows you to use the [`log`] macros [`error!`], [`warn!`],
//! [`info!`], [`debug!`] and [`trace!`] to print to stdout.
//!
//! It has no dependencies other than [`log`].  It doesn't print timestamps or
//! sequence numbers or print in color.  It just prints the message that was
//! logged.
//!
//! How to use it:
//! ```
//! use log::trace;
//! dumb_logger::init();
//! trace!("hello world");
//! ```
//!
//! If you want to change the max log level during init:
//! ```
//! use log::{trace, Level};
//! dumb_logger::init_with_level(Level::Info);
//! trace!("this won't print");
//! ```
//!
//! If you want to change the max log level at runtime:
//! ```
//! use log::{trace, Level};
//! dumb_logger::init_with_level(Level::Info);
//! dumb_logger::set_max(Level::Trace);
//! trace!("this will print");
//! ```
//!
//! [`log`]: https://docs.rs/log/0.4.8/log/
//! [`error!`]: https://docs.rs/log/0.4.8/log/macro.error.html
//! [`warn!`]: https://docs.rs/log/0.4.8/log/macro.warn.html
//! [`info!`]: https://docs.rs/log/0.4.8/log/macro.info.html
//! [`debug!`]: https://docs.rs/log/0.4.8/log/macro.debug.html
//! [`trace!`]: https://docs.rs/log/0.4.8/log/macro.trace.html


use log::{Level, Log, Metadata, Record};

struct DumbLogger;

impl Log for DumbLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // Not necessary; use log::set_max_level() instead.
        true
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
    }

    fn flush(&self) {}
}

static LOGGER: DumbLogger = DumbLogger;

/// Initialize logging with a max log level of Trace (everything
/// will be printed).
pub fn init() {
    init_with_level(Level::Trace);
}

/// Initialize logging with a specified max log level.
///
/// For example, if you only want Info and Error messages to be printed:
/// ```
/// dumb_logger::init_with_level(log::Level::Info);
/// ```
pub fn init_with_level(level: Level) {
    log::set_max_level(level.to_level_filter());
    log::set_logger(&LOGGER).unwrap();
}

/// Set the max log level.  Messages with a lower level will no
/// longer be printed.
pub fn set_max(level: Level) {
    log::set_max_level(level.to_level_filter());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        init();
        log::debug!("hello world");
    }
}
