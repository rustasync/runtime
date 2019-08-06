//! A [Tokio](https://docs.rs/tokio)-based asynchronous
//! [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(type_alias_impl_trait)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::{
    compat::Future01CompatExt,
    future::{BoxFuture, Future, FutureExt, TryFutureExt},
    task::SpawnError,
};
use lazy_static::lazy_static;
use tokio::timer::{Delay as TokioDelay, Interval as TokioInterval};

use std::io;
use std::net::SocketAddr;
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

/// The default Tokio runtime.
#[derive(Debug)]
pub struct Tokio;

#[derive(Debug)]
struct Compat<T>(T);

impl<T> Compat<T> {
    fn new(inner: T) -> Self {
        Self(inner)
    }

    fn get_ref(&self) -> &T {
        &self.0
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl runtime_raw::Runtime for Tokio {
    type TcpStream = impl runtime_raw::TcpStream;
    type TcpListener = impl runtime_raw::TcpListener<TcpStream = Self::TcpStream>;
    type UdpSocket = impl runtime_raw::UdpSocket;
    type Delay = impl runtime_raw::Delay;
    type Interval = impl runtime_raw::Interval;

    type ConnectTcpStream = impl Future<Output = io::Result<Self::TcpStream>> + Send;

    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        lazy_static! {
            static ref TOKIO_RUNTIME: tokio::runtime::Runtime = {
                tokio::runtime::Builder::new()
                    .after_start(|| {
                        runtime_raw::set_runtime(Tokio);
                    })
                    .build()
                    .unwrap()
            };
        }

        TOKIO_RUNTIME.executor().spawn(fut.unit_error().compat());
        Ok(())
    }

    fn connect_tcp_stream(&self, addr: &SocketAddr) -> Self::ConnectTcpStream {
        use futures01::Future;

        tokio::net::TcpStream::connect(addr)
            .map(Compat::new)
            .compat()
    }

    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Self::TcpListener> {
        tokio::net::TcpListener::bind(&addr).map(Compat::new)
    }

    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Self::UdpSocket> {
        tokio::net::UdpSocket::bind(&addr).map(Compat::new)
    }

    fn new_delay(&self, dur: Duration) -> Self::Delay {
        Compat::new(TokioDelay::new(Instant::now() + dur))
    }

    fn new_delay_at(&self, at: Instant) -> Self::Delay {
        Compat::new(TokioDelay::new(at))
    }

    fn new_interval(&self, dur: Duration) -> Self::Interval {
        Compat::new(TokioInterval::new(Instant::now(), dur))
    }
}

/// The single-threaded Tokio runtime based on `tokio-current-thread`.
#[derive(Debug)]
pub struct TokioCurrentThread;

impl runtime_raw::Runtime for TokioCurrentThread {
    type TcpStream = impl runtime_raw::TcpStream;
    type TcpListener = impl runtime_raw::TcpListener<TcpStream = Self::TcpStream>;
    type UdpSocket = impl runtime_raw::UdpSocket;
    type Delay = impl runtime_raw::Delay;
    type Interval = impl runtime_raw::Interval;

    type ConnectTcpStream = impl Future<Output = io::Result<Self::TcpStream>> + Send;

    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        lazy_static! {
            static ref TOKIO_RUNTIME: Mutex<tokio::runtime::current_thread::Handle> = {
                let (tx, rx) = mpsc::channel();

                thread::spawn(move || {
                    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();
                    let handle = rt.handle();
                    tx.send(handle).unwrap();

                    runtime_raw::set_runtime(TokioCurrentThread);
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

    fn connect_tcp_stream(&self, addr: &SocketAddr) -> Self::ConnectTcpStream {
        use futures01::Future;

        tokio::net::TcpStream::connect(addr)
            .map(Compat::new)
            .compat()
    }

    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Self::TcpListener> {
        tokio::net::TcpListener::bind(&addr).map(Compat::new)
    }

    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Self::UdpSocket> {
        tokio::net::UdpSocket::bind(&addr).map(Compat::new)
    }

    fn new_delay(&self, dur: Duration) -> Self::Delay {
        Compat::new(TokioDelay::new(Instant::now() + dur))
    }

    fn new_delay_at(&self, at: Instant) -> Self::Delay {
        Compat::new(TokioDelay::new(at))
    }

    fn new_interval(&self, dur: Duration) -> Self::Interval {
        Compat::new(TokioInterval::new(Instant::now(), dur))
    }
}
