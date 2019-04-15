#![feature(async_await, await_macro, futures_api)]

//! UDP echo server.
//!
//! To send messages do:
//! ```sh
//! $ nc -u localhost 8080
//! ```

use runtime::net::UdpSocket;

#[runtime::main]
async fn main() -> std::io::Result<()> {
    let mut socket = UdpSocket::bind("127.0.0.1:8080")?;
    let mut buf = vec![0u8; 1024];

    println!("Listening on {}", socket.local_addr()?);

    loop {
        let (recv, peer) = await!(socket.recv_from(&mut buf))?;
        let sent = await!(socket.send_to(&buf[..recv], &peer))?;
        println!("Sent {} out of {} bytes to {}", sent, recv, peer);
    }
}
