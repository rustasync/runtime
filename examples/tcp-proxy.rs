//! A TCP proxy server. Forwards connections from port 8081 to port 8080.

#![feature(async_await)]

use futures::prelude::*;
use futures::try_join;
use runtime::net::{TcpListener, TcpStream};

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:8081")?;
    println!("Listening on {}", listener.local_addr()?);

    // accept connections and process them in parallel
    listener
        .incoming()
        .try_for_each_concurrent(None, |client| {
            async move {
                runtime::task::spawn(async move {
                    let server = TcpStream::connect("127.0.0.1:8080").await?;
                    println!(
                        "Proxying {} to {}",
                        client.peer_addr()?,
                        server.peer_addr()?
                    );

                    let (cr, cw) = &mut client.split();
                    let (sr, sw) = &mut server.split();
                    let a = cr.copy_into(sw);
                    let b = sr.copy_into(cw);
                    try_join!(a, b)?;

                    Ok::<(), std::io::Error>(())
                })
                .await
            }
        })
        .await?;
    Ok(())
}
