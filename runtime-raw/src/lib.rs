//! Types for defining custom [Runtime](https://github.com/rustasync/runtime)s. See the
//! [Runtime](https://docs.rs/runtime) documentation for more details.
//!
//! These types are only necessary when implementing custom runtimes. If you're only trying to
//! perform IO, then there's no need to bother with any of these types as they will have been
//! implemented for you already.

#![feature(futures_api, async_await, await_macro)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::executor;
use futures::future::FutureObj;
use futures::prelude::*;
use futures::task::SpawnError;

use std::cell::Cell;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

mod tcp;
mod udp;

pub use tcp::*;
pub use udp::*;

thread_local! {
  static RUNTIME: Cell<Option<&'static dyn Runtime>> = Cell::new(None);
}

/// Get the current runtime.
#[inline]
pub fn current_runtime() -> &'static dyn Runtime {
    RUNTIME.with(|r| r.get().expect("the runtime has not been set"))
}

/// Set the current runtime.
///
/// This function must be called at the beginning of runtime's threads before they start polling
/// any futures.
pub fn set_runtime(runtime: &'static dyn Runtime) {
    RUNTIME.with(|r| {
        assert!(r.get().is_none(), "the runtime has already been set");
        r.set(Some(runtime))
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
        let t = await!(fut);
        let _ = tx.send(t);
    };

    rt.spawn_obj(FutureObj::from(Box::new(fut)))
        .expect("cannot spawn a future");

    executor::block_on(rx).expect("the main future has panicked")
}

/// The runtime trait.
pub trait Runtime: Send + Sync + 'static {
    /// Spawn a new future.
    fn spawn_obj(&self, fut: FutureObj<'static, ()>) -> Result<(), SpawnError>;

    /// Create a new `TcpStream`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `TcpStream` would prevent it from being a trait object.
    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn TcpStream>>>> + Send>>;

    /// Create a new `TcpListener`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `TcpListener` would prevent it from being a trait object.
    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Pin<Box<dyn TcpListener>>>;

    /// Create a new `UdpSocket`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `UdpSocket` would prevent it from being a trait object.
    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Pin<Box<dyn UdpSocket>>>;
}
