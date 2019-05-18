//! Runtime is what we imagine async APIs could look like if they were part of stdlib. We want async
//! Rust to be an experience that mirrors the quality of the standard lib. We believe that in order for
//! Rust to succeed it's not only important to make async Rust _possible_, it's crucial to make async
//! Rust feel _seamless_.
//!
//! And the embodiment of these values is Runtime: a library crafted to empower everyone to build
//! asynchronous software.
//!
//! - __runtime agnostic:__ Runtime comes with minimal OS bindings out of the box, but switching to a
//!     different runtime is a matter of changing a single line.
//! - __await anywhere:__ Runtime allows you to write async main functions, async tests, and async
//!     benchmarks. Experience what first-class async support in Rust feels like.
//! - __built for performance:__ Runtime is the thinnest layer possible on top of the backing
//!     implementations. All of the speed, none of the boilerplate.
//!
//! ## Examples
//! __UDP Echo Server__
//! ```no_run
//! #![feature(async_await)]
//!
//! use runtime::net::UdpSocket;
//!
//! #[runtime::main]
//! async fn main() -> std::io::Result<()> {
//!     let mut socket = UdpSocket::bind("127.0.0.1:8080")?;
//!     let mut buf = vec![0u8; 1024];
//!
//!     println!("Listening on {}", socket.local_addr()?);
//!
//!     loop {
//!         let (recv, peer) = socket.recv_from(&mut buf).await?;
//!         let sent = socket.send_to(&buf[..recv], &peer).await?;
//!         println!("Sent {} out of {} bytes to {}", sent, recv, peer);
//!     }
//! }
//! ```
//!
//! To send messages do:
//! ```sh
//! $ nc -u localhost 8080
//! ```
//!
//! __More Examples__
//! - [Hello World](https://github.com/rustasync/runtime/tree/master/examples/hello.rs)
//! - [Guessing Game](https://github.com/rustasync/runtime/blob/master/examples/guessing.rs)
//! - [TCP Echo Server](https://github.com/rustasync/runtime/blob/master/examples/tcp-echo.rs)
//! - [TCP Client](https://github.com/rustasync/runtime/tree/master/examples/tcp-client.rs)
//! - [TCP Proxy Server](https://github.com/rustasync/runtime/tree/master/examples/tcp-proxy.rs)
//! - [UDP Echo Server](https://github.com/rustasync/runtime/tree/master/examples/udp-echo.rs)
//! - [UDP Client](https://github.com/rustasync/runtime/tree/master/examples/udp-client.rs)
//!
//! ## Attributes
//! Runtime introduces 3 attributes to enable the use of await anywhere, and swap between different
//! runtimes. Each Runtime is bound locally to the initializing thread. This enables the testing of
//! different runtimes during testing or benchmarking.
//!
//! ```ignore
//! #[runtime::main]
//! async fn main() {}
//!
//! #[runtime::test]
//! async fn my_test() {}
//!
//! #[runtime::bench]
//! async fn my_bench() {}
//! ```
//!
//! ## Runtimes
//! Switching runtimes is a one-line change:
//!
//! ```ignore
//! /// Use the default Native Runtime
//! #[runtime::main]
//! async fn main() {}
//!
//! /// Use the Tokio Runtime
//! #[runtime::main(runtime_tokio::Tokio)]
//! async fn main() {}
//! ```
//!
//! The following backing runtimes are available:
//!
//! - [Runtime Native (default)](https://docs.rs/runtime-native) provides
//!   a thread pool, bindings to the OS, and a concurrent scheduler.
//! - [Runtime Tokio](https://docs.rs/runtime-tokio) provides a thread pool, bindings to the OS, and
//!   a work-stealing scheduler.

#![feature(async_await)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

pub mod net;
pub mod task;
pub mod time;

#[doc(inline)]
pub use task::spawn;

#[doc(inline)]
pub use runtime_attributes::{bench, test};

#[doc(inline)]
#[cfg(not(test))] // NOTE: exporting main breaks tests, we should file an issue.
pub use runtime_attributes::main;

#[doc(hidden)]
pub use runtime_raw as raw;

#[doc(hidden)]
#[cfg(feature = "native")]
pub use runtime_native as native;
