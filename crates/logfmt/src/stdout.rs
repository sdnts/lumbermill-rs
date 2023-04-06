use crate::log::{Log, LogFormat};
use std::io::{self, stdout, Stdout};

#[derive(Debug)]
pub struct StdoutLogger {
  stdout: Stdout,
}

impl StdoutLogger {
  pub fn new() -> Self {
    Self { stdout: stdout() }
  }

  pub fn log(&self, log: &Log, format: &LogFormat) -> io::Result<()> {
    let writer = &mut self.stdout.lock();
    log.write(writer, format)
  }
}
