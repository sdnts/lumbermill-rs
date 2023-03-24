pub struct Logger;
pub static LOGGER: Logger = Logger;

impl Logger {
  pub fn log(&self, kv: Vec<(String, String)>) {
    _ = kv;
  }
}
