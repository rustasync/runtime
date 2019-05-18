//! Extensions for Futures types.

use std::error::Error;
use std::fmt::{self, Debug, Display};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// The Future returned from [`FutureExt.timeout`].
#[derive(Debug)]
pub struct Timeout<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Future> Future for Timeout<T> {
    type Output = Result<<T as Future>::Output, TimeoutError>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!();
    }
}

/// The Error returned in the Future returned from [`FutureExt.timeout`].
pub struct TimeoutError(pub Instant);
impl Error for TimeoutError {}

impl Debug for TimeoutError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unimplemented!();
    }
}

impl Display for TimeoutError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unimplemented!();
    }
}

/// Extend `Future` with methods to time out execution.
pub trait FutureExt: Future + Sized {
    /// Timeout the future if it isn't completed after `dur` duration.
    fn timeout(self, _dur: Duration) -> Timeout<Self> {
        unimplemented!();
    }

    /// Timeout the future if it isn't completed by `at`.
    fn deadline(self, _at: Instant) -> Timeout<Self> {
        unimplemented!();
    }
}
