use crate::log::{Log, LogFormat, LogLevel};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::{
  fs::File,
  io::{stdout, BufWriter, Stdout, Write},
  path::PathBuf,
  sync::atomic::{AtomicUsize, Ordering},
};
use thiserror::Error;
use time::{Duration, OffsetDateTime, Time};

pub static LOGGER: OnceCell<Logger> = OnceCell::new();

#[derive(Debug, Error)]
pub enum InitError {
  #[error("The directory `{0}` does not exist")]
  DirectoryDoesNotExist(PathBuf),

  #[error("Loggers can only be initialized once")]
  MultipleInitializations,
}

#[derive(Debug)]
pub struct Logger {
  level: LogLevel,
  format: LogFormat,
  stdout: Option<Stdout>,
  directory: PathBuf,
  file: Option<Mutex<BufWriter<File>>>,
  roll_date: AtomicUsize,
}

impl Logger {
  fn create_file(&self, now: OffsetDateTime) -> File {
    let file = self.directory.join(format!("{}.log", now.date()));
    File::options()
      .create(true)
      .append(true)
      .open(file)
      .expect("Must have write access to file system")
  }

  fn roll_file(&self, file: &mut File) {
    let now = OffsetDateTime::now_utc();
    if now.unix_timestamp() as usize > self.roll_date.load(Ordering::Acquire) {
      *file = self.create_file(now);
      _ = self.roll_date.fetch_update(
        Ordering::Acquire,
        Ordering::Acquire,
        |_| {
          let roll_date: OffsetDateTime = now + Duration::DAY;
          let roll_date = roll_date.replace_time(Time::MIDNIGHT);
          let roll_date = roll_date.unix_timestamp() as usize;
          Some(roll_date)
        },
      );
    }
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

  pub fn stdout(mut self, s: bool) -> Self {
    if s {
      self.stdout = Some(stdout())
    } else {
      self.stdout = None;
    }

    self
  }

  pub fn file<Dir: Into<PathBuf>>(mut self, directory: Dir) -> Self {
    let directory: PathBuf = directory.into();
    self.directory = directory;
    self
  }

  pub fn init(mut self) -> Result<(), InitError> {
    if !self.directory.exists() {
      return Err(InitError::DirectoryDoesNotExist(self.directory.to_owned()));
    }

    let now = OffsetDateTime::now_utc();

    let file = self.create_file(now);
    let file = BufWriter::new(file);
    let file = Mutex::new(file);
    self.file = Some(file);

    LOGGER
      .set(self)
      .map_err(|_| InitError::MultipleInitializations)?;

    Ok(())
  }

  pub fn log(&self, log: Log) {
    if log.level < self.level {
      return;
    }

    if let Some(stdout) = &self.stdout {
      let writer = &mut stdout.lock();
      log
        .write(writer, &self.format)
        .expect("log write must not fail");
    }

    if let Some(file) = &self.file {
      let mut guard = file.lock();
      let writer = guard.get_mut();
      self.roll_file(writer);
      log
        .write(writer, &self.format)
        .expect("log write must not fail");
    }
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self {
      level: LogLevel::Info,
      format: LogFormat::Pretty,
      stdout: Some(stdout()),
      directory: PathBuf::from("logs"),
      file: None,
      roll_date: AtomicUsize::new(0),
    }
  }
}

impl Drop for Logger {
  fn drop(&mut self) {
    if let Some(file) = &mut self.file {
      _ = file.get_mut().flush();
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
