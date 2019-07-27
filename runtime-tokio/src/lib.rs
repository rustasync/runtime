//! A [Tokio](https://docs.rs/tokio)-based asynchronous
//! [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(async_await)]
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
use tokio::timer::{Delay as TokioDelay, Interval as TokioInterval};

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

use tcp::{TcpListener, TcpStream};
use time::{Delay, Interval};
use udp::UdpSocket;

// No matter how we "enter" the `BlockingRuntime` the `Runtime` interface to
// tokio is the same
struct TokioRuntime;

impl runtime_raw::Runtime for TokioRuntime {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        use tokio::executor::Executor;
        let mut e = tokio::executor::DefaultExecutor::current();
        e.spawn(Box::new(fut.unit_error().compat())).map_err(|e| {
            if e.is_shutdown() {
                SpawnError::shutdown()
            } else {
                panic!("can't handle tokio spawn error: {}", e);
            }
        })
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

/// The default Tokio runtime.
///
/// Uses a dedicated tokio instace to drive the runtime and cleans up afterwards.
#[derive(Debug)]
pub struct Tokio;

impl<F, T> runtime_raw::BlockingRuntime<F, T> for Tokio
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    fn block_on(&self, fut: F) -> T {
        let mut rt = tokio::runtime::Builder::new()
            .after_start(move || {
                runtime_raw::set_runtime(&TokioRuntime);
            })
            .build()
            .unwrap();

        runtime_raw::enter_runtime(&TokioRuntime, || {
            rt.block_on(fut.unit_error().boxed().compat()).unwrap()
        })
    }
}

/// The single-threaded Tokio runtime based on `tokio-current-thread`.
#[derive(Debug)]
pub struct TokioCurrentThread;

impl<F, T> runtime_raw::BlockingRuntime<F, T> for TokioCurrentThread
where
    F: Future<Output = T>,
{
    fn block_on(&self, fut: F) -> T {
        runtime_raw::enter_runtime(&TokioRuntime, move || {
            let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();

            rt.block_on(fut.unit_error().boxed_local().compat()).unwrap()
        })
    }
}
