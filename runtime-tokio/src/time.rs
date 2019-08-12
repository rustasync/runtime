use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::prelude::*;
use tokio::timer::{Delay as TokioDelay, Interval as TokioInterval};

#[derive(Debug)]
pub(crate) struct Delay {
    pub(crate) tokio_delay: TokioDelay,
}

impl runtime_raw::Delay for Delay {}

impl Future for Delay {
    type Output = Instant;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        futures::ready!(Pin::new(&mut self.tokio_delay).poll(cx));
        Poll::Ready(Instant::now())
    }
}

#[derive(Debug)]
pub(crate) struct Interval {
    pub(crate) tokio_interval: TokioInterval,
}

impl runtime_raw::Interval for Interval {}

impl Stream for Interval {
    type Item = Instant;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // https://docs.rs/tokio/0.1.20/tokio/timer/struct.Error.html
        Pin::new(&mut self.tokio_interval).poll_next(cx)
    }
}
