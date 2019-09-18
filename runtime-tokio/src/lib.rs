//! A [Tokio](https://docs.rs/tokio)-based asynchronous
//! [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::{
    compat::Future01CompatExt,
    future::{BoxFuture, FutureExt, TryFutureExt},
    task::SpawnError,
};
use lazy_static::lazy_static;
use tokio::timer::{Delay as TokioDelay, Interval as TokioInterval};

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

use tcp::{TcpListener, TcpStream};
use time::{Delay, Interval};
use udp::UdpSocket;

/// The default Tokio runtime.
#[derive(Debug)]
pub struct Tokio;

impl runtime_raw::Runtime for Tokio {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
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

        TOKIO_RUNTIME.executor().spawn(fut.unit_error().compat());
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> BoxFuture<'static, io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        use futures01::Future;

        let tokio_connect = tokio::net::TcpStream::connect(addr);
        let connect = tokio_connect.map(|tokio_stream| {
            Box::pin(TcpStream { tokio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
        });
        connect.compat().boxed()
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

    fn new_delay(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Delay>> {
        let tokio_delay = TokioDelay::new(Instant::now() + dur);
        Box::pin(Delay { tokio_delay })
    }

    fn new_delay_at(&self, at: Instant) -> Pin<Box<dyn runtime_raw::Delay>> {
        let tokio_delay = TokioDelay::new(at);
        Box::pin(Delay { tokio_delay })
    }

    fn new_interval(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Interval>> {
        let tokio_interval = TokioInterval::new(Instant::now(), dur);
        Box::pin(Interval { tokio_interval })
    }
}

/// The single-threaded Tokio runtime based on `tokio-current-thread`.
#[derive(Debug)]
pub struct TokioCurrentThread;

impl runtime_raw::Runtime for TokioCurrentThread {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
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
            .spawn(fut.unit_error().compat())
            .unwrap();
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> BoxFuture<'static, io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        use futures01::Future;

        let tokio_connect = tokio::net::TcpStream::connect(addr);
        let connect = tokio_connect.map(|tokio_stream| {
            Box::pin(TcpStream { tokio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
        });
        connect.compat().boxed()
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

    fn new_delay(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Delay>> {
        let tokio_delay = TokioDelay::new(Instant::now() + dur);
        Box::pin(Delay { tokio_delay })
    }

    fn new_delay_at(&self, at: Instant) -> Pin<Box<dyn runtime_raw::Delay>> {
        let tokio_delay = TokioDelay::new(at);
        Box::pin(Delay { tokio_delay })
    }

    fn new_interval(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Interval>> {
        let tokio_interval = TokioInterval::new(Instant::now(), dur);
        Box::pin(Interval { tokio_interval })
    }
}
