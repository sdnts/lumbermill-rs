//! Simple structured logging.
//!
//! This crate is a stop-gap till [`log::kv`](https://docs.rs/log/latest/log/kv/index.html) stabilizes. It marries [`tracing`](https://docs.rs/tracing)'s awesome
//! `event!` macro to `log`'s simplicity. The plan is to eventually drop the custom
//! macros in this crate and integrate with `log` directly.
//!
//! To start using this crate, initialize a logger as early as possible in your program,
//! then use the [`trace!`], [`debug!`], [`info!`], [`warn!`], [`error!`] or [`fatal!`]
//! macros to log messages and key-value pairs!
//!
//! ```
//! use lumbermill::{info , Logger};
//!
//! Logger::default().init(); // Use the default configuration
//!
//! info!(addr.ip = "0.0.0.0", uptime = 9001, "Server is listening on port {}", 3000);
//! ```
//!
//! Macro documentation delves deeper into what you can supply to them. Refer to
//! it for details.
//!
pub use time::OffsetDateTime;

mod file;
mod log;
mod logger;
mod macros;
mod stdout;

pub use file::RollInterval;
pub use log::{Log, LogFormat, LogLevel};
pub use logger::{Logger, LOGGER};

#[cfg(test)]
#[ctor::ctor]
fn test_init() {
  crate::Logger::default().init();
}
