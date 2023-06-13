use std::{
  fmt::{Arguments, Debug, Display},
  io,
};

use owo_colors::{OwoColorize, XtermColors};
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
  pub fn fg_color(&self) -> XtermColors {
    match self {
      Self::Trace => XtermColors::MalibuBlue,
      Self::Debug => XtermColors::BlazeOrange,
      Self::Info => XtermColors::UserGreen,
      Self::Warn => XtermColors::UserYellow,
      Self::Error => XtermColors::UserRed,
      Self::Fatal => XtermColors::UserRed,
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
  PrettyStructured,
  Compact,
  Json,
}

pub struct Log<'a> {
  pub timestamp: OffsetDateTime,
  pub level: LogLevel,
  pub kv: &'a [(&'static str, Arguments<'a>)],
  pub module: &'a str,
  pub file: &'a str,
  pub line: u32,
}

impl<'a> Log<'a> {
  pub fn write<Writer: io::Write>(
    &self,
    w: &mut Writer,
    format: &LogFormat,
  ) -> io::Result<()> {
    match format {
      LogFormat::Pretty => self.pretty(w),
      LogFormat::PrettyStructured => self.pretty_structured(w),
      LogFormat::Compact => self.compact(w),
      LogFormat::Json => self.json(w),
    }
  }
}

impl<'a> Log<'a> {
  fn pretty<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    let timestamp = format!(
      "{:0>2}:{:0>2}:{:0>2}.{:0>3}Z",
      self.timestamp.hour(),
      self.timestamp.minute(),
      self.timestamp.second(),
      self.timestamp.millisecond()
    );
    write!(w, "{} ", timestamp.dimmed())?;

    write!(w, "{} ", self.level.color(self.level.fg_color()))?;

    if self.level == LogLevel::Error || self.level == LogLevel::Fatal {
      write!(w, "{} ", message.1.red())?;
    } else {
      write!(w, "{} ", message.1)?;
    }

    kv.iter().try_for_each(|(k, v)| {
      write!(w, "{}{}{} ", k.dimmed(), "=".dimmed(), v)
    })?;

    write!(w, "{}{} ", "mod=".dimmed(), self.module)?;
    write!(w, "{}{}:{} ", "src=".dimmed(), self.file, self.line)?;

    writeln!(w)?;

    Ok(())
  }

  fn pretty_structured<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    let timestamp = format!(
      "{}T{:0>2}:{:0>2}:{:0>2}.{:0>3}Z",
      self.timestamp.date(),
      self.timestamp.hour(),
      self.timestamp.minute(),
      self.timestamp.second(),
      self.timestamp.millisecond()
    );
    write!(w, "{}{} ", "ts=".dimmed(), timestamp.dimmed())?;

    write!(
      w,
      "{}{:?} ",
      "level=".dimmed(),
      self.level.color(self.level.fg_color())
    )?;

    if self.level == LogLevel::Error || self.level == LogLevel::Fatal {
      write!(w, "{}\"{}\" ", "message=".dimmed(), message.1.red())?;
    } else {
      write!(w, "{}\"{}\" ", "message=".dimmed(), message.1)?;
    }

    kv.iter().try_for_each(|(k, v)| {
      write!(w, "{}{}{} ", k.dimmed(), "=".dimmed(), v.cyan())
    })?;

    let module = format!("mod={}", self.module);
    write!(w, "{} ", module.dimmed())?;
    let source = format!("src={}:{}", self.file, self.line);
    write!(w, "{} ", source.dimmed())?;

    writeln!(w)?;

    Ok(())
  }

  fn compact<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    // I'd like to use `time::format_description::well_known::Rfc3339` here but
    // it formats time with ns precision, I want ms precision.
    let t = &self.timestamp;
    write!(
      w,
      "ts={}T{:0>2}:{:0>2}:{:0>2}.{:0>3}Z ",
      t.date(),
      t.hour(),
      t.minute(),
      t.second(),
      t.millisecond()
    )?;

    write!(w, "level={:?} ", self.level)?;
    write!(w, "message=\"{}\" ", message.1)?;
    kv.iter().try_for_each(|(k, v)| write!(w, "{}={} ", k, v))?;
    write!(w, "mod={} ", self.module)?;
    write!(w, "src={}:{}", self.file, self.line)?;

    writeln!(w)?;

    Ok(())
  }

  fn json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
    // Because of the way our macros are set up, the KV list is ordered, which means
    // that the message will always be the last element
    let (message, kv) =
      self.kv.split_last().expect("A log message is required");

    write!(w, "{{",)?;

    // I'd like to use `time::format_description::well_known::Rfc3339` here but
    // it formats time with ns precision, I want ms precision.
    let t = &self.timestamp;
    write!(
      w,
      "\"ts\":\"{}T{:0>2}:{:0>2}:{:0>2}.{:0>3}Z\",",
      t.date(),
      t.hour(),
      t.minute(),
      t.second(),
      t.millisecond()
    )?;

    write!(w, "\"level\":\"{:?}\",", self.level)?;
    write!(w, "\"message\":\"{}\",", message.1)?;
    kv.iter()
      .try_for_each(|(k, v)| write!(w, "\"{}\":\"{}\",", k, v))?;
    write!(w, "\"mod\":\"{}\",", self.module)?;
    write!(w, "\"src\":\"{}:{}\"", self.file, self.line)?;

    write!(w, "}}")?;
    writeln!(w)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn compact() {
    let log = Log {
      timestamp: OffsetDateTime::UNIX_EPOCH,
      level: LogLevel::Info,
      kv: &[
        ("key1", format_args!("value1")),
        ("key1.2", format_args!("value1.2")),
        ("message", format_args!("logmsg")),
      ],
      module: "tests",
      file: "log.rs",
      line: 10,
    };
    let mut w: Vec<u8> = vec![];
    log.write(&mut w, &LogFormat::Compact).unwrap();
    assert_eq!(String::from_utf8(w).unwrap(), "ts=1970-01-01T00:00:00.000Z level=info message=\"logmsg\" key1=value1 key1.2=value1.2 mod=tests src=log.rs:10\n");
  }

  #[test]
  fn json() {
    let log = Log {
      timestamp: OffsetDateTime::UNIX_EPOCH,
      level: LogLevel::Info,
      kv: &[
        ("key1", format_args!("value1")),
        ("key1.2", format_args!("value1.2")),
        ("message", format_args!("logmsg")),
      ],
      module: "tests",
      file: "log.rs",
      line: 10,
    };
    let mut w: Vec<u8> = vec![];

    log.write(&mut w, &LogFormat::Json).unwrap();
    assert_eq!(String::from_utf8(w).unwrap(), "{\"ts\":\"1970-01-01T00:00:00.000Z\",\"level\":\"info\",\"message\":\"logmsg\",\"key1\":\"value1\",\"key1.2\":\"value1.2\",\"mod\":\"tests\",\"src\":\"log.rs:10\"}\n");
  }
}
