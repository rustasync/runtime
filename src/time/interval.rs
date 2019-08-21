use futures::prelude::*;

use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A stream representing notifications at a fixed interval.
#[must_use = "streams do nothing unless polled"]
pub struct Interval {
    inner: Pin<Box<dyn runtime_raw::Interval>>,
}

impl Interval {
    /// Create a stream that fires events at a set interval.
    ///
    /// ## Examples
    /// ```
    /// # use futures::prelude::*;
    /// use runtime::time::Interval;
    /// use std::time::{Duration, Instant};
    ///
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let start = Instant::now();
    /// let mut interval = Interval::new(Duration::from_millis(10)).take(3);
    /// while let Some(now) = interval.next().await {
    ///     println!("{}ms have elapsed", (now - start).as_millis());
    /// }
    ///
    /// assert!(Instant::now() - start >= Duration::from_millis(30));
    /// # Ok(())}
    /// ```
    #[inline]
    pub fn new(dur: Duration) -> Self {
        let inner = runtime_raw::current_runtime().new_interval(dur);
        Self { inner }
    }
}

impl Stream for Interval {
    type Item = Instant;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next_unpin(cx)
    }
}

impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.inner, f)
    }
}
