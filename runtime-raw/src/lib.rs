//! Types for defining custom [Runtime](https://github.com/rustasync/runtime)s. See the
//! [Runtime](https://docs.rs/runtime) documentation for more details.
//!
//! These types are only necessary when implementing custom runtimes. If you're only trying to
//! perform IO, then there's no need to bother with any of these types as they will have been
//! implemented for you already.

#![feature(async_await)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::future::BoxFuture;
use futures::prelude::*;
use futures::task::SpawnError;

use std::cell::Cell;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

pub use tcp::*;
pub use time::*;
pub use udp::*;

thread_local! {
    static RUNTIME: Cell<Option<&'static dyn Runtime>> = Cell::new(None);
}

/// Get the current runtime.
#[inline]
pub fn current_runtime() -> &'static dyn Runtime {
    RUNTIME.with(|r| r.get().expect("the runtime has not been set"))
}

/// Run passed function while passed runtime is set as current runtime.
pub fn enter_runtime<F, R>(runtime: &'static dyn Runtime, f: F) -> R
where
    F: FnOnce() -> R
{
    RUNTIME.with(move |r| {
        assert!(r.get().is_none(), "the runtime has already been set");

        struct Cleanup<'a>(&'a Cell<Option<&'static dyn Runtime>>);
        impl Drop for Cleanup<'_> {
            fn drop(&mut self) {
                self.0.set(None);
            }
        }
        let _cleanup = Cleanup(r);

        r.set(Some(runtime));
        f()
    })
}

/// Set the current runtime (per thread).
///
/// This function must be called at the beginning of runtime's threads before they start polling
/// any futures.
///
/// Setting the runtime can't be undone or changed again; use `enter_runtime` instead to only set
/// it for a scope.
pub fn set_runtime(runtime: &'static dyn Runtime) {
    RUNTIME.with(|r| {
        assert!(r.get().is_none(), "the runtime has already been set");
        r.set(Some(runtime))
    });
}

/// The runtime trait.
pub trait Runtime: Send + Sync + 'static {
    /// Spawn a new future.
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError>;

    /// Create a new `TcpStream`.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `TcpStream` would prevent it from being a trait object.
    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> BoxFuture<'static, io::Result<Pin<Box<dyn TcpStream>>>>;

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

    /// Create a new Future that wakes up after the given duration
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Delay` would prevent it from being a trait object.
    fn new_delay(&self, dur: Duration) -> Pin<Box<dyn Delay>>;

    /// Create a new Future that wakes up at the given time.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Delay` would prevent it from being a trait object.
    fn new_delay_at(&self, at: Instant) -> Pin<Box<dyn Delay>>;

    /// A stream representing notifications at a fixed interval.
    ///
    /// This method is defined on the `Runtime` trait because defining it on
    /// `Interval` would prevent it from being a trait object.
    fn new_interval(&self, dur: Duration) -> Pin<Box<dyn Interval>>;
}

/// Runtime trait for runtimes supporting blocking.
pub trait BlockingRuntime<F, T>
where
    F: Future<Output = T>,
{
    /// Runs a future inside the runtime and blocks on the result.
    ///
    /// Needs to call `enter_runtime` or `set_runtime` (only on background threads) in threads
    /// running futures.
    fn block_on(&self, fut: F) -> T;
}

/// Runs a future inside a runtime and blocks on the result.
pub fn enter<R, F, T>(rt: R, fut: F) -> T
where
    R: BlockingRuntime<F, T>,
    F: Future<Output = T>,
{
    rt.block_on(fut)
}
