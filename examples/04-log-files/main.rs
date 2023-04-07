use lumbermill::{info, Logger, RollInterval};
use std::fs::create_dir_all;

fn main() {
  // You can tell the Logger to log to a file (in addition to stdout).
  // To do this, you provide a directory where this log file will be stored. Note
  // that the logger does not create this directory, you must make sure it exists.
  create_dir_all("logs").unwrap();
  Logger::default()
    // This will create a file in the `logs` directory and log
    .file("./logs", RollInterval::None)
    // You can also supply a roll interval. An Hourly interval like below will
    // create one log file for every hour. This means you will end up with 24 separate
    // log files for every day your program runs. Los from 00:00AM - 00:59AM will be
    // in the first file, 01:00AM - 01:59AM will go to the second file, and so on.
    // Time is always recorded in UTC.
    // Note that you will generally only have a single `.file(...)` call. We have
    // multiple here for demonstration only.
    .file("./logs", RollInterval::Hourly)
    // Similarly you can roll files daily instead, which will create one log file
    // per day. There are other roll intervals available as well.
    .file("./logs", RollInterval::Daily)
    // It is also a good idea to change the log format. You generally don't want
    // terminal escape sequences in your log files.
    .compact()
    .init();

  info!("A log message");
}
