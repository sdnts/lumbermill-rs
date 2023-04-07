use lumbermill::{info, LogFormat, Logger};

fn main() {
  // The Logger is configurable and can log in one of three formats.
  // The default format is `Pretty`, which is suitable for logging to stdout.
  Logger::default()
    .format(LogFormat::Json) // Let's log in JSON instead
    .json() // This is a short-hand for the previous statement
    .init();

  info!(addr = "0.0.0.0", port = 7096, "Server is listening");
}
