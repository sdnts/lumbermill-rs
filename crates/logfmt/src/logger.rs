use crate::{log::Log, LogLevel};
use once_cell::sync::Lazy;
use std::io::stdout;

pub static MAX_LOG_LEVEL: LogLevel = LogLevel::Trace;

pub struct Logger {
  // format: "pretty" | "compact"
  // common_fields
  // file: File,
}
pub static LOGGER: Lazy<Logger> = Lazy::new(|| {
  // let dir = "logs";
  // let dir = "/var/log/toph";

  // fs::create_dir_all(dir).expect("Could not create log directory");
  // let file = File::options()
  //   .append(true)
  //   .create(true)
  //   .open(format!("{}/{}", dir, "node.log"))
  //   .expect("Could not open file");

  Logger {}
});

impl Logger {
  #[cfg(debug_assertions)]
  pub fn log(&self, log: Log) {
    _ = log.pretty(&mut stdout().lock());
    println!();
  }

  #[cfg(not(debug_assertions))]
  pub fn log(&mut self, log: Log) {
    _ = log.compact(&mut stdout().lock());
  }
}

#[cfg(test)]
mod tests {
  use crate::{log::Log, time::OffsetDateTime, LOGGER};

  #[test]
  fn stdout() {
    LOGGER.log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Trace,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("service", format_args!("{}", "toph")),
        ("node", format_args!("{}", "fra")),
        ("addr", format_args!("{}", "0.0.0.0")),
        ("port", format_args!("{}", "7096")),
        (
          "message",
          format_args!("Accepted TCP connection from 172.45.22.190:62498"),
        ),
      ],
    });

    LOGGER.log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Debug,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("service", format_args!("{}", "toph")),
        ("node", format_args!("{}", "fra")),
        ("addr", format_args!("{}", "0.0.0.0")),
        ("port", format_args!("{}", "7096")),
        ("message", format_args!("Established connection to DB")),
      ],
    });

    LOGGER.log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Info,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("service", format_args!("{}", "toph")),
        ("node", format_args!("{}", "fra")),
        ("addr", format_args!("{}", "0.0.0.0")),
        ("port", format_args!("{}", "7096")),
        ("message", format_args!("Listening on :7096")),
      ],
    });

    LOGGER.log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Warn,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("service", format_args!("{}", "toph")),
        ("node", format_args!("{}", "fra")),
        ("addr", format_args!("{}", "0.0.0.0")),
        ("port", format_args!("{}", "7096")),
        ("message", format_args!("Too many items in queue")),
      ],
    });

    LOGGER.log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Error,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("service", format_args!("{}", "toph")),
        ("node", format_args!("{}", "fra")),
        ("addr", format_args!("{}", "0.0.0.0")),
        ("port", format_args!("{}", "7096")),
        ("message", format_args!("Database connection dropped")),
      ],
    });
  }
}
