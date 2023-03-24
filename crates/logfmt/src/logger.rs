use std::io::{stdout, Write};

pub struct Logger;
pub static LOGGER: Logger = Logger;

impl Logger {
  pub fn log(&mut self, kv: Vec<(&'static str, String)>) {
    let mut stdout = stdout();

    kv.iter().for_each(|(k, v)| {
      _ = write!(stdout, "{}=\"{}\" ", k, v);
    });

    _ = writeln!(stdout);
  }
}
