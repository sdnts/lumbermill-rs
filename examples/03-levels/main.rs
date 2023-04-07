use lumbermill::{debug, error, fatal, info, trace, warn, LogLevel, Logger};

fn main() {
  // You can tell the logger to ignore log messages below a certain level.
  // There are 6 log levels, here they are, in order:
  // 1. Trace
  // 2. Debug
  // 3. Info
  // 4. Warn
  // 5. Error
  // 6. Fatal
  // Setting the minimum log level to any of these values disables all log levels
  // _lower_ that itself.
  Logger::default().level(LogLevel::Warn).init();

  trace!("I'm a trace log");
  debug!("I'm a debug log");
  info!("I'm a info log");
  warn!("I'm a warn log");
  error!("I'm a error log");
  fatal!("I'm a fatal log");

  // Running this example will print all logs except `trace`, `debug` & `info`
}
