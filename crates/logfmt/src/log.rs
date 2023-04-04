use std::{
  fmt::{Arguments, Debug, Display},
  io,
};
use time::OffsetDateTime;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
  Fatal,
}

impl LogLevel {
  pub fn color(&self) -> u8 {
    match self {
      Self::Trace => 81,
      Self::Debug => 202,
      Self::Info => 10,
      Self::Warn => 11,
      Self::Error => 9,
      Self::Fatal => 9,
    }
  }
}

impl Debug for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Trace => f.write_str("trace"),
      Self::Debug => f.write_str("debug"),
      Self::Info => f.write_str("info"),
      Self::Warn => f.write_str("warn"),
      Self::Error => f.write_str("error"),
      Self::Fatal => f.write_str("fatal"),
    }
  }
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Trace => f.write_str("TRC"),
      Self::Debug => f.write_str("DBG"),
      Self::Info => f.write_str("INF"),
      Self::Warn => f.write_str("WRN"),
      Self::Error => f.write_str("ERR"),
      Self::Fatal => f.write_str("FTL"),
    }
  }
}

#[derive(Debug)]
pub enum LogFormat {
  Pretty,
  Compact,
}

pub struct Log<'a> {
  pub timestamp: OffsetDateTime,
  pub level: LogLevel,
  pub module: &'a str,
  pub file: &'a str,
  pub line: u32,
  pub kv: &'a [(&'static str, Arguments<'a>)],
}

impl<'a> Log<'a> {
  pub fn pretty<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    write!(
      w,
      "\x1B[38;5;244m{:0>2}:{:0>2}:{:0>2}.{:0>3}Z ",
      self.timestamp.hour(),
      self.timestamp.minute(),
      self.timestamp.second(),
      self.timestamp.millisecond()
    )?;

    write!(w, "\x1B[38;5;{}m{} ", self.level.color(), self.level)?;

    if self.level == LogLevel::Error {
      write!(w, "\x1B[38;5;9m{} ", message.1)?;
    } else {
      write!(w, "\x1B[0m{} ", message.1)?;
    }

    kv.iter()
      .try_for_each(|(k, v)| write!(w, "\x1B[38;5;243m{}=\x1B[0m{} ", k, v))?;

    write!(w, "\x1B[38;5;243mmod=\x1B[0m{} ", self.module)?;
    write!(w, "\x1B[38;5;243msrc=\x1B[0m{}:{} ", self.file, self.line)?;

    write!(w, "\x1B[0m")?;

    Ok(())
  }

  pub fn compact<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    use time::format_description::well_known::Rfc3339;
    write!(w, "ts=",)?;
    self
      .timestamp
      .format_into(w, &Rfc3339)
      .expect("Timestamp is invalid. This is a bug.");
    write!(w, " ",)?;

    write!(w, "lvl={:?} ", self.level)?;
    write!(w, "msg=\"{}\" ", message.1)?;
    kv.iter().try_for_each(|(k, v)| write!(w, "{}={} ", k, v))?;
    write!(w, "mod={} ", self.module)?;
    write!(w, "src={}:{} ", self.file, self.line)?;

    Ok(())
  }
}
