use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{
  fs::{self, File},
  io::{stdout, Write},
};

pub struct Logger {
  // format: "pretty" | "compact"
  // common_fields
  file: File,
}
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
  #[cfg(debug_assertions)]
  let dir = "logs";
  #[cfg(not(debug_assertions))]
  let dir = "/var/log/toph";

  fs::create_dir_all(dir).expect("Could not create log directory");
  let file = File::options()
    .append(true)
    .create(true)
    .open(format!("{}/{}", dir, "node.log"))
    .expect("Could not open file");

  Mutex::new(Logger { file })
});

impl Logger {
  pub fn log(&mut self, kv: Vec<(&'static str, String)>) {
    let mut stdout = stdout();

    kv.iter().for_each(|(k, v)| {
      _ = write!(stdout, "{}=\"{}\" ", k, v);
      _ = write!(self.file, "{}=\"{}\" ", k, v);
    });

    _ = writeln!(stdout);
    _ = writeln!(self.file);
  }
}
