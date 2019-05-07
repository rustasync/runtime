//! Same as guessing game from the book but over TCP rather than standard IO.
//!
//! https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//!
//! To communicate with this server do:
//! ```sh
//! $ nc localhost 8080
//! ```

#![feature(async_await, await_macro)]

use futures::prelude::*;
use rand::Rng;
use runtime::net::{TcpListener, TcpStream};
use std::cmp::Ordering;
use std::str;

async fn play(stream: TcpStream) -> Result<(), failure::Error> {
    println!("Accepting from: {}", stream.peer_addr()?);

    let (reader, writer) = &mut stream.split();
    let mut buf = vec![0u8; 1024];

    await!(writer.write_all(b"Guess the number!\n"))?;

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        await!(writer.write_all(b"Please input your guess.\n"))?;

        let len = await!(reader.read(&mut buf))?;
        if len == 0 {
            return Ok(());
        }

        let guess = str::from_utf8(&buf[..len])?;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let msg = format!("You guessed: {}\n", guess);
        await!(writer.write_all(msg.as_bytes()))?;

        match guess.cmp(&secret_number) {
            Ordering::Less => await!(writer.write_all(b"Too small!\n"))?,
            Ordering::Greater => await!(writer.write_all(b"Too big!\n"))?,
            Ordering::Equal => {
                await!(writer.write_all(b"You win!\n"))?;
                break;
            }
        }
    }

    Ok(())
}

#[runtime::main]
async fn main() -> Result<(), failure::Error> {
    let mut listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on {}", &listener.local_addr()?);

    let incoming = listener.incoming().map_err(|e| e.into());
    await!(incoming.try_for_each_concurrent(!0, async move |stream| {
        await!(runtime::spawn(play(stream)))?;
        Ok::<(), failure::Error>(())
    }))?;
    Ok(())
}
