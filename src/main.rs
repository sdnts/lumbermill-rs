use anyhow::Result;
use std::{io::Write, net::TcpListener};

fn main() -> Result<()> {
    let location = hostname::get()?;
    let location = location.to_string_lossy();
    let addr = "0.0.0.0:7096";

    let listener = TcpListener::bind(addr)?;
    let response = format!(
        "HTTP/1.1 200 OK
Connection: close
Content-Type: text/html

<html>
    <head>
        <title>Hello World!</title>
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

"
    );

    println!("Listening on {addr}");

    for stream in listener.incoming() {
        let mut stream = stream?;
        stream.write_all(response.as_bytes())?;
        stream.shutdown(std::net::Shutdown::Both)?;
    }

    Ok(())
}
