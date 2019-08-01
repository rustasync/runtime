//! A simple echo server.
//!
//! Run the server and connect to it with `nc 127.0.0.1 8080`.
//! The server will wait for you to enter lines of text and then echo them back.

#![feature(async_await)]

use futures::prelude::*;
use runtime::net::TcpListener;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them in parallel
    listener
        .incoming()
        .try_for_each_concurrent(None, |stream| {
            async move {
                runtime::task::spawn(async move {
                    println!("Accepting from: {}", stream.peer_addr()?);

                    let (reader, writer) = &mut stream.split();
                    reader.copy_into(writer).await?;
                    Ok::<(), std::io::Error>(())
                })
                .await
            }
        })
        .await?;
    Ok(())
}
