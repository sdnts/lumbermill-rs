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
    let file = create_file(now, &interval, &directory);
    let roll_date = next_roll_date(now, &interval).unix_timestamp() as usize;

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
    let now = log.timestamp;

    if roll_date == 0 {
      return log.write(file, format);
    }

    if now.unix_timestamp() as usize > roll_date {
      *file = create_file(now, &self.interval, &self.directory);

      // It is essential to not drop the file MutexGuard here to make sure we set
      // the `next_roll_date` only once, and correctly.

      _ = self.roll_date.fetch_update(
        Ordering::Acquire,
        Ordering::Acquire,
        |_| {
          let date = next_roll_date(now, &self.interval);
          Some(date.unix_timestamp() as usize)
        },
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

fn create_file(
  now: OffsetDateTime,
  interval: &RollInterval,
  directory: &Path,
) -> File {
  let filename = match interval {
    RollInterval::None => String::from("log.log"),
    _ => {
      let now = round_date(now, interval);
      format!(
        "{}T{:0>2}-{:0>2}-{:0>2}.log",
        now.date(),
        now.hour(),
        now.minute(),
        now.second()
      )
    }
  };

  File::options()
    .create(true)
    .append(true)
    .open(directory.join(filename))
    .expect("Must have write access to log file")
}

fn next_roll_date(
  now: OffsetDateTime,
  interval: &RollInterval,
) -> OffsetDateTime {
  match interval {
    RollInterval::None => now,
    RollInterval::Secondly => now + Duration::SECOND,
    RollInterval::Minutely => now + Duration::MINUTE,
    RollInterval::Hourly => now + Duration::HOUR,
    RollInterval::Daily => now + Duration::DAY,
  }
}

fn round_date(date: OffsetDateTime, interval: &RollInterval) -> OffsetDateTime {
  match interval {
    RollInterval::None => date,
    RollInterval::Secondly => date,
    RollInterval::Minutely => date.replace_second(0).unwrap(),
    RollInterval::Hourly => {
      date.replace_minute(0).unwrap().replace_second(0).unwrap()
    }
    RollInterval::Daily => date.replace_time(Time::MIDNIGHT),
  }
}
