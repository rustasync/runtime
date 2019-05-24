//! A TCP client
//!
//! Sends user input to a server on port 8080, and echoes the response. To
//! spawn the server, do:
//! ```sh
//! $ cargo run --example tcp-echo
//! ```

#![feature(async_await)]

use futures::compat::Compat01As03;
use futures::prelude::*;
use futures::try_join;
use runtime::net::TcpStream;
use tokio::io::{stdin, stdout};

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), failure::Error> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to {}", &stream.peer_addr()?);

    let (reader, writer) = &mut stream.split();

    let mut input = Compat01As03::new(stdin());
    let mut output = Compat01As03::new(stdout());

    println!("Write a message to send");

    let a = input.copy_into(writer);
    let b = reader.copy_into(&mut output);

    try_join!(a, b)?;

    Ok(())
}
