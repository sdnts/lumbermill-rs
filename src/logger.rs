use crate::{
  file::FileLogger,
  log::{Log, LogFormat, LogLevel},
  stdout::StdoutLogger,
  RollInterval,
};
use std::{path::PathBuf, sync::OnceLock};

pub static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
pub struct Logger {
  level: LogLevel,
  format: LogFormat,
  stdout: Option<StdoutLogger>,
  file: Option<FileLogger>,
}

impl Logger {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn builder() -> Self {
    Self::default()
  }
}

impl Logger {
  pub fn level(mut self, level: LogLevel) -> Self {
    self.level = level;
    self
  }

  pub fn format(mut self, format: LogFormat) -> Self {
    self.format = format;
    self
  }

  pub fn pretty(self) -> Self {
    self.format(LogFormat::Pretty)
  }

  pub fn compact(self) -> Self {
    self.format(LogFormat::Compact)
  }

  pub fn json(self) -> Self {
    self.format(LogFormat::Json)
  }

  pub fn stdout(mut self, s: bool) -> Self {
    if s {
      self.stdout = Some(StdoutLogger::new())
    } else {
      self.stdout = None;
    }

    self
  }

  pub fn file<Dir: Into<PathBuf>>(
    mut self,
    directory: Dir,
    roll_interval: RollInterval,
  ) -> Self {
    self.file = Some(FileLogger::new(directory, roll_interval));
    self
  }

  pub fn init(self) {
    LOGGER
      .set(self)
      .expect("Loggers can only be initialized once");
  }

  pub fn log(&self, log: Log) {
    if log.level < self.level {
      return;
    }

    _ = self
      .stdout
      .as_ref()
      .map(|logger| logger.log(&log, &self.format));

    _ = self
      .file
      .as_ref()
      .map(|logger| logger.log(&log, &self.format));
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self {
      level: LogLevel::Info,
      format: LogFormat::Pretty,
      stdout: Some(StdoutLogger::new()),
      file: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{log::Log, OffsetDateTime, LOGGER};

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
        ("addr", format_args!("{}", "https://postgres.org")),
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
        ("addr", format_args!("{}", "0.0.0.0")),
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
        ("count", format_args!("{}", "9001")),
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
        ("reason", format_args!("{}", "No connectivity")),
        ("message", format_args!("Database connection dropped")),
      ],
    });

    LOGGER.get().unwrap().log(Log {
      timestamp: OffsetDateTime::now_utc(),
      level: crate::LogLevel::Fatal,
      module: module_path!(),
      file: file!(),
      line: line!(),
      kv: &[
        ("usage", format_args!("{}", "128MB")),
        ("message", format_args!("Out of memory")),
      ],
    });
  }
}
