use anyhow::Result;
use lumbermill::{info, trace, LogLevel, Logger};
use std::{io::Write, net::TcpListener};

fn main() -> Result<()> {
  #[cfg(debug_assertions)]
  Logger::default().level(LogLevel::Trace).pretty().init();

  #[cfg(not(debug_assertions))]
  {
    let dir = "logs";
    std::fs::create_dir_all(dir)?;
    Logger::default()
      .level(LogLevel::Trace)
      .compact()
      .file(dir, lumbermill::RollInterval::Daily)
      .init();
  }

  let location = hostname::get()?;
  let location = location.to_string_lossy();

  let addr = "0.0.0.0";
  let port = 7096;

  let listener = TcpListener::bind(format!("{addr}:{port}"))?;
  let response = format!(
    "HTTP/1.1 200 OK
Connection: close
Content-Type: text/html

<html>
  <head>
    <title>Hello World!</title>
    <link rel=\"icon\" type=\"image/svg+xml\" href=\"https://sdnts.dev/favicon.svg\">
    <style>
      body {{
        font-family: JetBrains Mono, Menlo, ui-monospace, monospace;
        display: flex;
        align-items: center;
        justify-content: center;
      }}
    </style>
  </head>
  <body>
    <h1>Hello from {location}!</h1>
  </body>
</html>

");

  info!(addr, port, "Listening");

  for stream in listener.incoming() {
    let mut stream = stream?;
    let ip = stream.peer_addr()?.ip();
    trace!(ip, "Incoming connection");

    stream.write_all(response.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Both)?;
  }

  Ok(())
}
