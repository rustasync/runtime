//! A [Tokio](https://docs.rs/tokio)-based asynchronous
//! [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(async_await, await_macro, futures_api)]
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
use std::sync::{mpsc, Mutex};
use std::thread;

mod tcp;
mod udp;

use tcp::{TcpListener, TcpStream};
use udp::UdpSocket;

/// The default Tokio runtime.
#[derive(Debug)]
pub struct Tokio;

impl runtime_raw::Runtime for Tokio {
    fn spawn_obj(&self, fut: FutureObj<'static, ()>) -> Result<(), SpawnError> {
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

        TOKIO_RUNTIME
            .executor()
            .spawn(Compat::new(fut.map(|_| Ok(()))));
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> + Send>>
    {
        use futures::compat::Compat01As03;
        use futures01::Future;

        let tokio_connect = tokio::net::TcpStream::connect(addr);
        let connect = tokio_connect.map(|tokio_stream| {
            Box::pin(TcpStream { tokio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
        });
        Box::pin(Compat01As03::new(connect))
    }

    fn bind_tcp_listener(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::TcpListener>>> {
        let tokio_listener = tokio::net::TcpListener::bind(&addr)?;
        Ok(Box::pin(TcpListener { tokio_listener }))
    }

    fn bind_udp_socket(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::UdpSocket>>> {
        let tokio_socket = tokio::net::UdpSocket::bind(&addr)?;
        Ok(Box::pin(UdpSocket { tokio_socket }))
    }
}

/// The single-threaded Tokio runtime based on `tokio-current-thread`.
#[derive(Debug)]
pub struct TokioCurrentThread;

impl runtime_raw::Runtime for TokioCurrentThread {
    fn spawn_obj(&self, fut: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        lazy_static! {
            static ref TOKIO_RUNTIME: Mutex<tokio::runtime::current_thread::Handle> = {
                let (tx, rx) = mpsc::channel();

                thread::spawn(move || {
                    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
                    let handle = rt.handle();
                    tx.send(handle).unwrap();

                    runtime_raw::set_runtime(&TokioCurrentThread);
                    let forever = futures01::future::poll_fn(|| {
                        Ok::<futures01::Async<()>, ()>(futures01::Async::NotReady)
                    });
                    rt.block_on(forever).unwrap();
                });

                let handle = rx.recv().unwrap();
                Mutex::new(handle)
            };
        }

        TOKIO_RUNTIME
            .lock()
            .unwrap()
            .spawn(Compat::new(fut.map(|_| Ok(()))))
            .unwrap();
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> + Send>>
    {
        use futures::compat::Compat01As03;
        use futures01::Future;

        let tokio_connect = tokio::net::TcpStream::connect(addr);
        let connect = tokio_connect.map(|tokio_stream| {
            Box::pin(TcpStream { tokio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
        });
        Box::pin(Compat01As03::new(connect))
    }

    fn bind_tcp_listener(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::TcpListener>>> {
        let tokio_listener = tokio::net::TcpListener::bind(&addr)?;
        Ok(Box::pin(TcpListener { tokio_listener }))
    }

    fn bind_udp_socket(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::UdpSocket>>> {
        let tokio_socket = tokio::net::UdpSocket::bind(&addr)?;
        Ok(Box::pin(UdpSocket { tokio_socket }))
    }
}
