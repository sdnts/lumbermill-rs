use crate::log::{Log, LogFormat, LogLevel};
use once_cell::sync::OnceCell;
use std::io::stdout;

pub static LOGGER: OnceCell<Logger> = OnceCell::new();

#[derive(Debug)]
pub struct Logger {
  format: LogFormat,
  min_level: LogLevel,
}

impl Logger {
  pub fn level(mut self, level: LogLevel) -> Self {
    self.min_level = level;
    self
  }

  pub fn format(mut self, format: LogFormat) -> Self {
    self.format = format;
    self
  }

  pub fn pretty(mut self) -> Self {
    self.format = LogFormat::Pretty;
    self
  }

  pub fn compact(mut self) -> Self {
    self.format = LogFormat::Compact;
    self
  }

  pub fn init(self) {
    LOGGER
      .set(self)
      .expect("Logger can only be initialized once");
  }

  pub fn log(&self, log: Log) {
    if log.level < self.min_level {
      return;
    }

    {
      let writer = &mut stdout().lock();
      match self.format {
        LogFormat::Pretty => {
          log.pretty(writer).expect("log write must not fail")
        }
        LogFormat::Compact => {
          log.compact(writer).expect("log write must not fail")
        }
      }
      println!(); // Flush stdout buffer
    }
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self {
      format: LogFormat::Pretty,
      min_level: LogLevel::Info,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{log::Log, time::OffsetDateTime, LOGGER};

  #[test]
  fn stdout() {
    LOGGER.get().unwrap().log(Log {
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

    LOGGER.get().unwrap().log(Log {
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

    LOGGER.get().unwrap().log(Log {
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

    LOGGER.get().unwrap().log(Log {
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

    LOGGER.get().unwrap().log(Log {
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
