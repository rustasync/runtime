//! Types and Functions for working with asynchronous tasks.

use futures::future::{FutureObj, RemoteHandle};
use futures::prelude::*;
use futures::task::{Spawn, SpawnError};

/// A [`Spawn`] handle to runtime's thread pool for spawning futures.
///
/// This allows integrating runtime with libraries based on explicitly passed spawners.
#[derive(Debug)]
pub struct Spawner {
    _reserved: (),
}

impl Spawner {
    /// Construct a new [`Spawn`] handle.
    pub fn new() -> Self {
        Self { _reserved: () }
    }
}

impl Default for Spawner {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Spawner {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Spawn for Spawner {
    fn spawn_obj(&mut self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        (&*self).spawn_obj(future)
    }
}

impl<'a> Spawn for &'a Spawner {
    fn spawn_obj(&mut self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        runtime_raw::current_runtime().spawn_boxed(future.boxed())
    }
}

/// Spawn a future on the runtime's thread pool.
///
/// This function can only be called after a runtime has been initialized.
///
/// # Examples
///
/// ```
/// #![feature(async_await)]
///
/// #[runtime::main]
/// async fn main() {
///     runtime::task::spawn(async {
///         // might not run at all as we're not waiting for it to be done
///         println!("running the future");
///     });
/// }
/// ```
pub fn spawn<F>(fut: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    runtime_raw::current_runtime()
        .spawn_boxed(fut.boxed())
        .expect("cannot spawn a future");
}

/// Spawns a future on the runtime's thread pool and makes the result available.
///
/// This function can only be called after a runtime has been initialized.
///
/// If the returned handle is dropped the future is aborted by default.
///
/// ```
/// #![feature(async_await)]
///
/// #[runtime::main]
/// async fn main() {
///     let handle = runtime::task::spawn_remote(async {
///         println!("running the future");
///         42
///     });
///     assert_eq!(handle.await, 42);
/// }
/// ```
pub fn spawn_remote<F, T>(fut: F) -> RemoteHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (fut, handle) = fut.remote_handle();

    runtime_raw::current_runtime()
        .spawn_boxed(fut.boxed())
        .expect("cannot spawn a future");

    handle
}
