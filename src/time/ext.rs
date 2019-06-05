//! Extensions for Futures types.

use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use super::Delay;

/// The Future returned from [`FutureExt`].
///
/// [`FutureExt.timeout`]: trait.FutureExt.html
#[derive(Debug)]
pub struct Timeout<F: Future> {
    future: F,
    delay: Delay,
}

impl<F: Future> Timeout<F> {
    pin_utils::unsafe_pinned!(future: F);
    pin_utils::unsafe_pinned!(delay: Delay);
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, TimeoutError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().future().poll(cx) {
            Poll::Pending => {}
            Poll::Ready(t) => return Poll::Ready(Ok(t)),
        }

        if self.as_mut().poll(cx).is_ready() {
            let err = Err(TimeoutError(Instant::now()));
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}

/// The Error returned from [`Timeout`].
///
/// [`Timeout`]: struct.Timeout.html
#[derive(Debug)]
pub struct TimeoutError(pub Instant);
impl Error for TimeoutError {}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

/// Extend `Future` with methods to time out execution.
pub trait FutureExt: Future + Sized {
    /// Creates a new future which will take at most `dur` time to resolve from
    /// the point at which this method is called.
    ///
    /// This combinator creates a new future which wraps the receiving future
    /// in a timeout. The future returned will resolve in at most `dur` time
    /// specified (relative to when this function is called).
    ///
    /// If the future completes before `dur` elapses then the future will
    /// resolve with that item. Otherwise the future will resolve to an error
    /// once `dur` has elapsed.
    ///
    /// # Examples
    /// ```
    /// # #![feature(async_await)]
    /// # use futures::prelude::*;
    /// use std::time::Duration;
    /// use runtime::time::FutureExt;
    ///
    /// # fn long_future() -> impl Future<Output = std::io::Result<()>> {
    /// #     futures::future::ok(())
    /// # }
    /// #
    /// #[runtime::main]
    /// async fn main() {
    ///     let future = long_future();
    ///     let timed_out = future.timeout(Duration::from_millis(100));
    ///
    ///     match timed_out.await {
    ///         Ok(item) => println!("got {:?} within enough time!", item),
    ///         Err(_) => println!("took too long to produce the item"),
    ///     }
    /// }
    /// ```
    fn timeout(self, dur: Duration) -> Timeout<Self> {
        Timeout {
            delay: Delay::new(dur),
            future: self,
        }
    }

    /// Creates a new future which will resolve no later than `at` specified.
    ///
    /// This method is otherwise equivalent to the [`timeout`] method except that
    /// it tweaks the moment at when the timeout elapsed to being specified with
    /// an absolute value rather than a relative one. For more documentation see
    /// the [`timeout`] method.
    ///
    /// [`timeout`]: trait.FutureExt.html#method.timeout
    ///
    /// # Examples
    /// ```
    /// # #![feature(async_await)]
    /// # use futures::prelude::*;
    /// use std::time::{Duration, Instant};
    /// use runtime::time::FutureExt;
    ///
    /// # fn long_future() -> impl Future<Output = std::io::Result<()>> {
    /// #     futures::future::ok(())
    /// # }
    /// #
    /// #[runtime::main]
    /// async fn main() {
    ///     let future = long_future();
    ///     let at = Instant::now() + Duration::from_millis(100);
    ///     let timed_out = future.timeout_at(at);
    ///
    ///     match timed_out.await {
    ///         Ok(item) => println!("got {:?} within enough time!", item),
    ///         Err(_) => println!("took too long to produce the item"),
    ///     }
    /// }
    /// ```
    fn timeout_at(self, at: Instant) -> Timeout<Self> {
        Timeout {
            delay: Delay::new_at(at),
            future: self,
        }
    }
}

impl<T: Future> FutureExt for T {}
