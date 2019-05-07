//! A simple echo server.
//!
//! Run the server and connect to it with `nc 127.0.0.1 8080`.
//! The server will wait for you to enter lines of text and then echo them back.

#![feature(async_await, await_macro, stmt_expr_attributes, proc_macro_hygiene)]

use futures::prelude::*;
use runtime::net::TcpListener;
use runtime::for_await;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", listener.local_addr()?);

    #[for_await]
    for stream in listener.incoming() {
        println!("Accepting from: {}", stream.peer_addr()?);

        let (reader, writer) = &mut stream.split();
        await!(reader.copy_into(writer))?;
    }
}
