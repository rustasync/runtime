//! A TCP client
//!
//! Sends "hello world" to a server on port 8080, and echoes the response. To
//! spawn the server, do:
//! ```sh
//! $ cargo run --example tcp-echo
//! ```

#![feature(async_await, await_macro, futures_api)]

use futures::prelude::*;
use runtime::net::TcpStream;

#[runtime::main]
async fn main() -> Result<(), failure::Error> {
    let mut stream = await!(TcpStream::connect("127.0.0.1:8080"))?;
    println!("Connected to {}", &stream.peer_addr()?);

    let msg = "hello world";
    println!("<- {}", msg);
    await!(stream.write_all(msg.as_bytes()))?;

    let mut buf = vec![0u8; 1024];
    await!(stream.read(&mut buf))?;
    println!("-> {}\n", String::from_utf8(buf)?);

    Ok(())
}
