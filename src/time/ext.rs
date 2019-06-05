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
    /// Time out the future if it isn't completed after `dur` duration.
    fn timeout(self, dur: Duration) -> Timeout<Self> {
        Timeout {
            delay: Delay::new(dur),
            future: self,
        }
    }

    /// Time out the future if it isn't completed before `at`.
    fn timeout_at(self, at: Instant) -> Timeout<Self> {
        Timeout {
            delay: Delay::new_at(at),
            future: self,
        }
    }
}
