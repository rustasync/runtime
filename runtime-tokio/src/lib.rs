//! A [Tokio](https://docs.rs/tokio)-based asynchronous
//! [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(async_await, await_macro, futures_api)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::{
    compat::Compat,
    future::{Future, FutureExt, FutureObj},
    task::SpawnError,
};
use lazy_static::lazy_static;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

mod tcp;
mod udp;

use tcp::{TcpListener, TcpStream};
use udp::UdpSocket;

lazy_static! {
    static ref TOKIO_RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new()
            .after_start(|| {
                runtime_raw::set_runtime(&Tokio);
            })
            .build()
            .unwrap()
    };
}

/// The Tokio runtime.
#[derive(Debug)]
pub struct Tokio;

impl runtime_raw::Runtime for Tokio {
    fn spawn_obj(&self, fut: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        TOKIO_RUNTIME
            .executor()
            .spawn(Compat::new(fut.map(|_| Ok(()))));
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> Pin<Box<dyn Future<Output = io::Result<Box<dyn runtime_raw::TcpStream>>> + Send>> {
        use futures::compat::Compat01As03;
        use futures01::Future;

        let tokio_connect = tokio::net::TcpStream::connect(addr);
        let connect = tokio_connect.map(|tokio_stream| {
            Box::new(TcpStream { tokio_stream }) as Box<dyn runtime_raw::TcpStream>
        });
        Box::pin(Compat01As03::new(connect))
    }

    fn bind_tcp_listener(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Box<dyn runtime_raw::TcpListener>> {
        let tokio_listener = tokio::net::TcpListener::bind(&addr)?;
        Ok(Box::new(TcpListener { tokio_listener }))
    }

    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Box<dyn runtime_raw::UdpSocket>> {
        let tokio_socket = tokio::net::UdpSocket::bind(&addr)?;
        Ok(Box::new(UdpSocket { tokio_socket }))
    }
}
