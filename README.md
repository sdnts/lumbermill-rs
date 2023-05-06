# 🪓 lumbermill

[![Crates.io](https://img.shields.io/crates/v/lumbermill.svg)](https://crates.io/crates/lumbermill)
[![Documentation](https://docs.rs/lumbermill/badge.svg)](https://docs.rs/lumbermill)

Simple structured logging.

![Screenshot of log output](https://raw.githubusercontent.com/sdnts/lumbermill-rs/main/examples/logs.png)

### Usage

```rust
use lumbermill::{info, Logger}

fn main() {
  // Initialize the logger early
  Logger::default().init();

  // Log a single line
  info!("Incoming connection from {:?} {}", addr, port);

  // Attach key-value pairs with the log message
  info!(addr.ip = ip, addr.port = port, "Listening on {}", port);
  // Or use the shorthand if the key's name is the same as the variable:
  info!(addr.ip, port, "Listening on {}", port);

  // Attach key-value pairs with the log message, formatting them using their
  // `Debug` trait (useful when variables do not implement `Display`)
  info!(addr.ip = ?ip, addr.port = port, "Listening on {}", port);
  // Or in the shorthand notation:
  info!(?addr.ip, port, "Listening on {}", port);
}
```

The `trace!`, `debug!`, `info!`, `warn!`, `error!` & `fatal!` are heavily inspired
by `tracing`'s macros because they're good.

The default logger prints pretty logs to `stdout` only, but you can configure the `Logger` to behave differently:

```rust
use lumbermill::{LogFormat, LogLevel, RollInterval};

Logger::builder()
  .format(LogFormat::Compact) // Set the format of logs
  .level(LogLevel::Info) // Set the minimum log level
  .stdout(false) // Stop printing to stdout
  .file("./logs", RollInterval::Daily) // Log to a directory; one file per day

  // Shorthands
  .pretty() //  .format(LogFormat::Pretty)
  .compact() // .format(LogFormat::Compact)
  .json() // .format(LogFormat::Json)

  // Remember to call `init` after configuration!
  .init();
```

You can have different active configurations in different scenarios by using the
`#![cfg]` macro:

```rust
// Pretty logs on stdout during development
#[cfg(debug_assertions)]
Logger::default().level(LogLevel::Trace).pretty().init();

// Compact logs on rolling files in production
#[cfg(not(debug_assertions))]
{
  let dir = "./logs";
  std::fs::create_dir_all(dir)?;
  Logger::default()
    .level(LogLevel::Info)
    .compact()
    .file(dir, lumbermill::RollInterval::Hourly)
    .init();
}
```

### Examples

[Examples](https://github.com/sdnts/lumbermill-rs/tree/main/examples) are a good entrypoint to learn about the library. Run them this way:

```sh
$ cargo run --example 01-defaults # Or replace this wil a different example's name
```

[Docs](https://docs.rs/lumbermill) usually go into more detail once you get the hang of things.

### Why?

The `tracing` ecosystem is awesome, but it's also overkill for a lot of apps who
only need structured logging and not a distributed tracing solution. The `log` crate
is the obvious alternative, but its `kv` module is a work-in-progress.
You are also unable to log key-value pairs that do not implement `Display` in
a incovenient way.

This crate is a stop-gap till `log::kv` stabilizes. It marries `tracing`'s awesome
`event!` macro to `log`'s simplicity. The plan is to eventually drop the custom
macros in this crate and integrate with `log` directly.

### MSRV

This crate currently requires a `nightly` version of Rust, and will till Rust v1.70, when `OnceLock` gets stabilized.

### Credits

- [tracing](https://docs.rs/tracing)
- [tracing_appender](https://docs.rs/tracing_appender)
- [log](https://docs.rs/log)
