use futures::prelude::*;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A stream representing notifications at a fixed interval.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Interval {
    inner: Pin<Box<dyn runtime_raw::Interval>>,
}

impl Interval {
    /// Create a stream that fires events at a set interval.
    #[inline]
    pub fn new(dur: Duration) -> Self {
        let inner = runtime_raw::new_interval(dur);
        Self { inner }
    }
}

impl Stream for Interval {
    type Item = Instant;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}
