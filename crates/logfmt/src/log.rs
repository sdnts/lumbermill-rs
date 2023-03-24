use std::fmt::Display;

#[derive(Debug)]
pub enum LogLevel {
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}

impl Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Trace => f.write_str("trace"),
      Self::Debug => f.write_str("debug"),
      Self::Info => f.write_str("info"),
      Self::Warn => f.write_str("warn"),
      Self::Error => f.write_str("error"),
    }
  }
}
