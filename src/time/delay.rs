use futures::prelude::*;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A future representing the notification that an elapsed duration has occurred.
#[must_use = "futures do nothing unless awaited"]
#[derive(Debug)]
pub struct Delay {
    inner: Pin<Box<dyn runtime_raw::Delay>>,
}

impl Delay {
    /// Sleep the current future for the given duration.
    #[inline]
    pub fn new(dur: Duration) -> Self {
        let inner = runtime_raw::new_delay(dur);
        Self { inner }
    }

    /// Sleep the current future until the given time.
    #[inline]
    pub fn new_at(at: Instant) -> Self {
        let inner = runtime_raw::new_delay_at(dur);
        Self { inner }
    }
}

impl Future for Delay {
    type Output = Instant;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}
