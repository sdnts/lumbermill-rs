/// Log at the `Trace` level.
///
/// Accepts a format string (and arguments) that are considered the line that you
/// wish to log. Optionally, you may also provide a set of key-value pairs that
/// will be associated with this log line. These key-value pairs must all appear
/// before the format string.
/// This API is heavily inspired by [`tracing`'s `event!` macro](https://docs.rs/tracing/latest/tracing/macro.trace.html)
///
/// # Examples
/// ```
/// use logfmt::trace;
///
/// #[derive(Debug)]
/// struct Addr {
///   ip: &'static str,
///   port: usize
/// };
///
/// let ip = "0.0.0.0";
/// let port = 7096;
/// let addr = Addr { ip, port };
///
/// // Log a single line
/// trace!("Incoming connection from {:?} {}", addr, port);
///
/// // Attach key-value pairs with the log message
/// trace!(addr.ip = ip, addr.port = port, "Listening on {}", port);
/// // Or use the shorthand if the key's name is the same as the variable:
/// trace!(addr.ip, port, "Listening on {}", port);
///
///
/// // Attach key-value pairs with the log message, formatting them using their
/// // `Debug` trait (useful when variables do not implement `Display`)
/// trace!(addr.ip = ?ip, addr.port = port, "Listening on {}", port);
/// // Or in the shorthand notation:
/// trace!(?addr.ip, port, "Listening on {}", port);
/// ```
#[macro_export]
macro_rules! trace {
  ($($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Trace, $($k).+ = $v, $($fields)*)
  };
  ($($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Trace, $($k).+ = ?$v, $($fields)*)
  };
  ($($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Trace, $($k).+, $($fields)*)
  };
  (?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Trace, ?$($k).+, $($fields)*)
  };
  ($($msg:tt)+) => {
    $crate::__internal_log!($crate::LogLevel::Trace, $($msg)+)
  };
}

/// Log at the `Debug` level
///
/// This functions exactly the same way as [`trace!`], refer to it for details
/// and examples.
#[macro_export]
macro_rules! debug {
  ($($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Debug, $($k).+ = $v, $($fields)*)
  };
  ($($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Debug, $($k).+ = ?$v, $($fields)*)
  };
  ($($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Debug, $($k).+, $($fields)*)
  };
  (?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Debug, ?$($k).+, $($fields)*)
  };
  ($($msg:tt)+) => {
    $crate::__internal_log!($crate::LogLevel::Debug, $($msg)+)
  };
}

/// Log at the `Info` level
///
/// This functions exactly the same way as [`trace!`], refer to it for details
/// and examples.
#[macro_export]
macro_rules! info {
  ($($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Info, $($k).+ = $v, $($fields)*)
  };
  ($($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Info, $($k).+ = ?$v, $($fields)*)
  };
  ($($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Info, $($k).+, $($fields)*)
  };
  (?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Info, ?$($k).+, $($fields)*)
  };
  ($($msg:tt)+) => {
    $crate::__internal_log!($crate::LogLevel::Info, $($msg)+)
  };
}

/// Log at the `Warn` level
///
/// This functions exactly the same way as [`trace!`], refer to it for details
/// and examples.
#[macro_export]
macro_rules! warn {
  ($($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Warn, $($k).+ = $v, $($fields)*)
  };
  ($($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Warn, $($k).+ = ?$v, $($fields)*)
  };
  ($($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Warn, $($k).+, $($fields)*)
  };
  (?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Warn, ?$($k).+, $($fields)*)
  };
  ($($msg:tt)+) => {
    $crate::__internal_log!($crate::LogLevel::Warn, $($msg)+)
  };
}

/// Log at the `Error` level
///
/// This functions exactly the same way as [`trace!`], refer to it for details
/// and examples.
#[macro_export]
macro_rules! error {
  ($($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Error, $($k).+ = $v, $($fields)*)
  };
  ($($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Error, $($k).+ = ?$v, $($fields)*)
  };
  ($($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Error, $($k).+, $($fields)*)
  };
  (?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!($crate::LogLevel::Error, ?$($k).+, $($fields)*)
  };
  ($($msg:tt)+) => {
    $crate::__internal_log!($crate::LogLevel::Error, $($msg)+)
  };
}

/// Internal-only, do not use directly. All public macros converge here.
#[doc(hidden)]
#[macro_export]
macro_rules! __internal_log {
  // kv muncher
  ({ $($kv:expr),* }, $($k:ident).+ = $v:expr, $($fields:tt)*) => {
    $crate::__internal_log!({ $($kv,)* (stringify!($($k).+), format!("{}", $v)) }, $($fields)*)
  };
  ({ $($kv:expr),* }, $($k:ident).+ = ?$v:expr, $($fields:tt)*) => {
    $crate::__internal_log!({ $($kv,)* (stringify!($($k).+), format!("{:?}", $v)) }, $($fields)*)
  };
  ({ $($kv:expr),* }, $($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!({ $($kv,)* (stringify!($($k).+), format!("{}", $($k).+)) }, $($fields)*)
  };
  ({ $($kv:expr),* }, ?$($k:ident).+, $($fields:tt)*) => {
    $crate::__internal_log!({ $($kv,)* (stringify!($($k).+), format!("{:?}", $($k).+)) }, $($fields)*)
  };
  ({ $($kv:expr),* }, $($msg:tt)*) => {
    $crate::__internal_log!({ $($kv,)* ("message", format!($($msg)*)) })
  };
  ({ $($kv:expr),* }) => {
    vec![ $($kv,)* ]
  };

  // entrypoint
  ($lvl:expr, $($fields:tt)+) => {
    $crate::LOGGER.log($crate::__internal_log!({
      ("level", format!("{}", $lvl)),
      ("file", format!("{}:{}", file!(), line!())),
      ("mod", format!("{}", module_path!()))
    }, $($fields)*));
  };
}

mod tests {
  struct Addr<'a> {
    ip: &'a str,
    port: usize,
  }

  #[test]
  fn message_only() {
    let ip = "0.0.0.0";
    let port = 7096;

    trace!("Message");
    trace!("Message {ip}");
    trace!("Message {ip} {port}");
    trace!("Message {}", ip);
    trace!("Message {} {}", ip, port);
  }

  #[test]
  fn debug_message_only() {
    let ip = "0.0.0.0";
    let port = 7096;

    trace!("Message {ip:?}");
    trace!("Message {ip:?} {port:?}");
    trace!("Message {:?}", ip);
    trace!("Message {:?} {:?}", ip, port);
  }

  #[test]
  fn field_shorthand() {
    let ip = "0.0.0.0";
    let port = 7096;
    let addr = Addr { ip, port };

    trace!(ip, "Message {}", ip);
    trace!(ip, "Message {} {}", ip, port);
    trace!(ip, port, "Message {}", ip);
    trace!(ip, port, "Message {} {}", ip, port);

    trace!(addr.ip, "Message {}", ip);
    trace!(addr.ip, "Message {} {}", addr.ip, port);
    trace!(addr.ip, port, "Message {}", addr.ip);
    trace!(addr.ip, port, "Message {} {}", addr.ip, port);
  }

  #[test]
  fn debug_field_shorthand() {
    let ip = "0.0.0.0";
    let port = 7096;
    let addr = Addr { ip, port };

    trace!(?ip, "Message {}", ip);
    trace!(?ip, "Message {} {}", ip, port);
    trace!(?ip, ?port, "Message {}", ip);
    trace!(?ip, ?port, "Message {} {}", ip, port);

    trace!(?addr.ip, "Message {}", addr.ip);
    trace!(?addr.ip, "Message {} {}", addr.ip, port);
    trace!(?addr.ip, ?port, "Message {}", addr.ip);
    trace!(?addr.ip, ?port, "Message {} {}", addr.ip, port);
  }

  #[test]
  fn fields() {
    let ip = "0.0.0.0";
    let port = 7096;
    let addr = Addr { ip, port };

    trace!(ip = "127.0.0.1", "Message");
    trace!(ip = "127.0.0.1", port = "7096", "Message");
    trace!(ip = ip, "Message");
    trace!(ip = ip, port = port, "Message");

    trace!(ip = addr.ip, "Message");
    trace!(ip = addr.ip, port = addr.port, "Message");
  }

  #[test]
  fn debug_fields() {
    let ip = "0.0.0.0";
    let port = 7096;
    let addr = Addr { ip, port };

    trace!(ip = ?ip, "Message");
    trace!(ip = ?ip, port = ?port, "Message");

    trace!(ip = ?addr.ip, "Message");
    trace!(ip = ?addr.ip, port = ?addr.port, "Message");
  }

  #[test]
  fn mixed() {
    let ip = "0.0.0.0";
    let port = 7096;
    let addr = Addr { ip, port };

    trace!(
      ip,
      ?port,
      addr.ip = ?addr.ip,
      addr.port = addr.port,
      addr_ip = ?addr.ip,
      addr_port = addr.port,
      "Message {ip}:{port}"
    );
  }
}
