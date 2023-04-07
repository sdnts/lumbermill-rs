use lumbermill::{info, Logger};

#[derive(Debug)]
struct Addr<'a> {
  ip: &'a str,
  port: usize,
}

fn main() {
  // Initialize the logger with default options
  // Logger initializations should ideally be done very early in your program.
  // Logger can be initialized exactly once. `lumbermill` will panic otherwise.
  Logger::default().init();

  let ip = "0.0.0.0";
  let port = 7096;
  let addr = Addr { ip, port };

  // Log a single message.
  info!("Incoming connection from {:?} {}", addr, port);

  // Attach key-value pairs with the log message. Key-value pairs must all appear
  // before the log message.
  info!(ip = ip, port = port, "Listening on {}", port);

  // Key names can be any valid Rust identifier. This means `.` are fair game
  info!(ip = ip, addr.port = addr.port, "Listening on {}", port);

  // If a key's name is the same as the variable it references, you can shorthand
  // the macro call. The following is equivalent to the previous call.
  info!(ip, port, "Listening on {}", port);

  // Sometimes you want to log a variable that does not implement `Display`.
  // Prefix a key-value pair's value with a `?` to tell `lumbermill` to use the
  // `Debug` implementation of the variable
  info!(addr = ?addr, port = ?port, "Listening on {}", port);

  // This also works with the shorthand notation
  info!(?addr.ip, port, "Listening on {}", port);
}
