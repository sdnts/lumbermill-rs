use crate::log::{Log, LogFormat};
use parking_lot::Mutex;
use std::{
  fs::File,
  io::{self, BufWriter, Write},
  path::{Path, PathBuf},
  sync::atomic::{AtomicUsize, Ordering},
};
use time::{Duration, OffsetDateTime, Time};

#[derive(Debug)]
pub enum RollInterval {
  None,
  Secondly,
  Minutely,
  Hourly,
  Daily,
}

#[derive(Debug)]
pub struct FileLogger {
  directory: PathBuf,
  file: Mutex<BufWriter<File>>,
  interval: RollInterval,
  roll_date: AtomicUsize,
}

impl FileLogger {
  pub fn new<Dir>(directory: Dir, interval: RollInterval) -> Self
  where
    Dir: Into<PathBuf>,
  {
    let directory: PathBuf = directory.into();
    let now = OffsetDateTime::now_utc();
    let roll_date = next_roll_date(&interval, now);
    let file = create_file(&directory, roll_date);

    Self {
      directory,
      file: Mutex::new(BufWriter::new(file)),
      interval,
      roll_date: AtomicUsize::new(roll_date),
    }
  }

  pub fn log(&self, log: &Log, format: &LogFormat) -> io::Result<()> {
    let mut guard = self.file.lock();
    let file = guard.get_mut();
    let roll_date = self.roll_date.load(Ordering::Acquire);

    if roll_date == 0 {
      return log.write(file, format);
    }

    let now = log.timestamp;

    if now.unix_timestamp() as usize > roll_date {
      // Update file handle
      *file = create_file(&self.directory, roll_date);

      // It is essential to not drop the file MutexGuard here to make sure we set
      // the `next_roll_date` only once, and correctly.

      // Set new next_roll_date
      _ = self.roll_date.fetch_update(
        Ordering::Acquire,
        Ordering::Acquire,
        |_| Some(next_roll_date(&self.interval, now)),
      );
    }

    log.write(file, format)
  }
}

impl Drop for FileLogger {
  fn drop(&mut self) {
    _ = self.file.get_mut().flush();
  }
}

fn create_file(directory: &Path, roll_date: usize) -> File {
  let now = OffsetDateTime::from_unix_timestamp(roll_date as i64).unwrap();
  let file = directory.join(format!(
    "{}T{:0>2}-{:0>2}-{:0>2}.log",
    now.date(),
    now.hour(),
    now.minute(),
    now.second()
  ));

  File::options()
    .create(true)
    .append(true)
    .open(file)
    .expect("Must have write access to log file")
}

fn next_roll_date(interval: &RollInterval, now: OffsetDateTime) -> usize {
  match interval {
    RollInterval::None => 0,
    RollInterval::Secondly => {
      let roll_date: OffsetDateTime = now + Duration::SECOND;
      roll_date.unix_timestamp() as usize
    }
    RollInterval::Minutely => {
      let roll_date: OffsetDateTime = now + Duration::MINUTE;
      let roll_date = roll_date.replace_second(0).unwrap();
      roll_date.unix_timestamp() as usize
    }
    RollInterval::Hourly => {
      let roll_date: OffsetDateTime = now + Duration::HOUR;
      let roll_date = roll_date.replace_minute(0).unwrap();
      let roll_date = roll_date.replace_second(0).unwrap();
      roll_date.unix_timestamp() as usize
    }
    RollInterval::Daily => {
      let roll_date: OffsetDateTime = now + Duration::DAY;
      let roll_date = roll_date.replace_time(Time::MIDNIGHT);
      roll_date.unix_timestamp() as usize
    }
  }
}
