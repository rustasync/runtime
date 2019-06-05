use futures::prelude::*;

use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A future representing the notification that an elapsed duration has occurred.
#[must_use = "futures do nothing unless awaited"]
pub struct Delay {
    inner: Pin<Box<dyn runtime_raw::Delay>>,
}

impl Delay {
    /// Continue execution after the duration has passed.
    ///
    /// ## Examples
    /// ```
    /// # #![feature(async_await)]
    /// use runtime::time::Delay;
    /// use std::time::{Duration, Instant};
    ///
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let start = Instant::now();
    /// let now = Delay::new(Duration::from_millis(40)).await;
    ///
    /// assert!(now - start >= Duration::from_millis(40));
    /// # Ok(())}
    /// ```
    #[inline]
    pub fn new(dur: Duration) -> Self {
        let inner = runtime_raw::current_runtime().new_delay(dur);
        Self { inner }
    }

    /// Continue execution after the given instant.
    ///
    /// ## Examples
    /// ```
    /// # #![feature(async_await)]
    /// use runtime::time::Delay;
    /// use std::time::{Duration, Instant};
    ///
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let start = Instant::now();
    /// let now = Delay::new_at(start + Duration::from_millis(40)).await;
    ///
    /// assert!(now - start >= Duration::from_millis(40));
    /// # Ok(())}
    /// ```
    #[inline]
    pub fn new_at(at: Instant) -> Self {
        let inner = runtime_raw::current_runtime().new_delay_at(at);
        Self { inner }
    }
}

impl fmt::Debug for Delay {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unimplemented!();
    }
}

impl Future for Delay {
    type Output = Instant;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}
