#![feature(async_await, await_macro, futures_api)]

//! UDP client.
//!
//! To start an echo server do:
//! ```sh
//! $ cargo run --example udp-echo
//! ```

use runtime::net::UdpSocket;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("127.0.0.1:8081")?;
    println!("Listening on {}", socket.local_addr()?);

    let msg = "hello world";
    println!("<- {}", msg);
    await!(socket.send_to(msg.as_bytes(), "127.0.0.1:8080"))?;

    let mut buf = vec![0u8; 1024];
    await!(socket.recv_from(&mut buf))?;
    println!("-> {}\n", String::from_utf8_lossy(&mut buf));

    Ok(())
}
