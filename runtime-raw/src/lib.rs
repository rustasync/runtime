//! Types for defining custom [Runtime](https://github.com/rustasync/runtime)s. See the
//! [Runtime](https://docs.rs/runtime) documentation for more details.
//!
//! These types are only necessary when implementing custom runtimes. If you're only trying to
//! perform IO, then there's no need to bother with any of these types as they will have been
//! implemented for you already.

#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::executor;
use futures::future::BoxFuture;
use futures::prelude::*;
use futures::task::SpawnError;

use std::cell::Cell;
use std::io;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

mod into_dyn;
mod tcp;
mod time;
mod udp;

pub use tcp::*;
pub use time::*;
pub use udp::*;

/// A dynamic runtime that returns type-erased implementations.
pub type DynRuntime = dyn Runtime<
    TcpStream = BoxTcpStream,
    TcpListener = BoxTcpListener,
    UdpSocket = BoxUdpSocket,
    Delay = BoxDelay,
    Interval = BoxInterval,
    ConnectTcpStream = BoxFuture<'static, io::Result<BoxTcpStream>>,
>;

thread_local! {
    static RUNTIME: Cell<Option<&'static DynRuntime>> = Cell::new(None);
}

/// Get the current runtime.
#[inline]
pub fn current_runtime() -> &'static DynRuntime {
    RUNTIME.with(|r| r.get().expect("the runtime has not been set"))
}

/// Set the current runtime.
///
/// This function must be called at the beginning of runtime's threads before they start polling
/// any futures.
pub fn set_runtime(runtime: impl Runtime) {
    RUNTIME.with(|r| {
        assert!(r.get().is_none(), "the runtime has already been set");
        r.set(Some(Box::leak(Box::new(into_dyn::Dyn::new(runtime)))))
    });
}

/// Runs a future inside a runtime and blocks on the result.
pub fn enter<R, F, T>(rt: R, fut: F) -> T
where
    R: Runtime,
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = futures::channel::oneshot::channel();

    let fut = async move {
        let t = fut.await;
        let _ = tx.send(t);
    };

    rt.spawn_boxed(fut.boxed()).expect("cannot spawn a future");

    executor::block_on(rx).expect("the main future has panicked")
}

/// The runtime trait.
pub trait Runtime: Send + Sync + 'static {
    /// The [`TcpStream`] implementation for this runtime.
    type TcpStream: TcpStream;

    /// The [`TcpListener`] implementation for this runtime.
    type TcpListener: TcpListener<TcpStream = Self::TcpStream>;

    /// The [`UdpSocket`] implementation for this runtime.
    type UdpSocket: UdpSocket;

    /// The [`Delay`] implementation for this runtime.
    type Delay: Delay;

    /// The [`Interval`] implementation for this runtime.
    type Interval: Interval;

    /// The [`Future`] implementation for this runtime's [`connect_tcp_stream`](Runtime::connect_tcp_stream) function.
    type ConnectTcpStream: Future<Output = io::Result<Self::TcpStream>> + Send;

    /// Spawn a new future.
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError>;

    /// Create a new `TcpStream`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `TcpStream` would prevent it from being a trait object.
    fn connect_tcp_stream(&self, addr: &SocketAddr) -> Self::ConnectTcpStream;

    /// Create a new `TcpListener`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `TcpListener` would prevent it from being a trait object.
    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Self::TcpListener>;

    /// Create a new `UdpSocket`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `UdpSocket` would prevent it from being a trait object.
    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Self::UdpSocket>;

    /// Create a new Future that wakes up after the given duration
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Delay` would prevent it from being a trait object.
    fn new_delay(&self, dur: Duration) -> Self::Delay;

    /// Create a new Future that wakes up at the given time.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Delay` would prevent it from being a trait object.
    fn new_delay_at(&self, at: Instant) -> Self::Delay;

    /// A stream representing notifications at a fixed interval.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Interval` would prevent it from being a trait object.
    fn new_interval(&self, dur: Duration) -> Self::Interval;
}
