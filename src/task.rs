//! Types and Functions for working with asynchronous tasks.

use std::pin::Pin;

use futures::future::FutureObj;
use futures::prelude::*;
use futures::task::{Context, Poll};

/// Spawn a future on the runtime's thread pool.
///
/// This function can only be called after a runtime has been initialized.
///
/// # Examples
///
/// ```
/// #![feature(async_await, await_macro)]
///
/// #[runtime::main]
/// async fn main() {
///     let handle = runtime::spawn(async {
///         println!("running the future");
///         42
///     });
///     assert_eq!(await!(handle), 42);
/// }
/// ```
pub fn spawn<F, T>(fut: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = futures::channel::oneshot::channel();

    let fut = async move {
        let t = await!(fut);
        let _ = tx.send(t);
    };

    runtime_raw::current_runtime()
        .spawn_obj(FutureObj::from(Box::new(fut)))
        .expect("cannot spawn a future");

    JoinHandle { rx }
}

/// A handle that awaits the result of a [`spawn`]ed future.
///
/// [`spawn`]: fn.spawn.html
#[derive(Debug)]
pub struct JoinHandle<T> {
    pub(crate) rx: futures::channel::oneshot::Receiver<T>,
}

impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.rx.poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(t)) => Poll::Ready(t),
            Poll::Ready(Err(_)) => panic!(), // TODO: Is this OK? Print a better error message?
        }
    }
}
