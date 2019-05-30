use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::prelude::*;
use futures_timer::{Delay as AsyncDelay, Interval as AsyncInterval};

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

// TODO: implement this
impl fmt::Debug for Delay {
    // fmt::Display::fmt(self.async_delay, f)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Delay")
            .field("when", &"...")
            .finish()
    }
}

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

// TODO: implement this
impl fmt::Debug for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // fmt::Display::fmt(self.async_interval, f)
        f.debug_struct("Interval")
            .field("delay", &"...")
            .field("interval", &"...")
            .finish()
    }
}
