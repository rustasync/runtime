//! A simple echo server.
//!
//! Run the server and connect to it with `nc 127.0.0.1 8080`.
//! The server will wait for you to enter lines of text and then echo them back.

#![feature(async_await, await_macro, futures_api)]

use futures::prelude::*;
use runtime::net::TcpListener;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them in parallel
    let mut incoming = listener.incoming();
    while let Some(stream) = await!(incoming.next()) {
        runtime::spawn(async move {
            let stream = stream?;
            println!("Accepting from: {}", stream.peer_addr()?);

            let (reader, writer) = &mut stream.split();
            await!(reader.copy_into(writer))?;
            Ok::<(), std::io::Error>(())
        });
    }
    Ok(())
}
