//! A TCP client
//!
//! Sends "hello world" to a server on port 8080, and echoes the response. To
//! spawn the server, do:
//! ```sh
//! $ cargo run --example tcp-echo
//! ```

use futures::prelude::*;
use runtime::net::TcpStream;

#[runtime::main]
async fn main() -> Result<(), failure::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to {}", &stream.peer_addr()?);

    let msg = "hello world";
    println!("<- {}", msg);
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = vec![0u8; 1024];
    stream.read(&mut buf).await?;
    println!("-> {}\n", String::from_utf8(buf)?);

    Ok(())
}
