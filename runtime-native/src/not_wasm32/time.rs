use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::prelude::*;
use futures::ready;
use futures_timer::{Delay as AsyncDelay, Interval as AsyncInterval};

use super::Compat;

impl runtime_raw::Delay for Compat<AsyncDelay> {}

impl Future for Compat<AsyncDelay> {
    type Output = Instant;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        ready!(self.get_pin_mut().poll(cx)).unwrap();
        Poll::Ready(Instant::now())
    }
}

impl runtime_raw::Interval for Compat<AsyncInterval> {}

impl Stream for Compat<AsyncInterval> {
    type Item = Instant;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        ready!(self.get_pin_mut().poll_next(cx)).unwrap();
        Poll::Ready(Some(Instant::now()))
    }
}
