use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::prelude::*;
use futures_timer::{Delay as AsyncDelay, Interval as AsyncInterval};

#[derive(Debug)]
pub(crate) struct Delay {
    pub(crate) async_delay: AsyncDelay,
}

impl runtime_raw::Delay for Delay {}

impl Future for Delay {
    type Output = Instant;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        futures::ready!(Pin::new(&mut self.async_delay).poll(cx)).unwrap();
        Poll::Ready(Instant::now())
    }
}

#[derive(Debug)]
pub(crate) struct Interval {
    pub(crate) async_interval: AsyncInterval,
}

impl runtime_raw::Interval for Interval {}

impl Stream for Interval {
    type Item = Instant;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        futures::ready!(Pin::new(&mut self.async_interval).poll_next(cx)).unwrap();
        Poll::Ready(Some(Instant::now()))
    }
}
