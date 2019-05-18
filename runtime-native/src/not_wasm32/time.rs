use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::prelude::*;
use futures_timer::Delay as AsyncDelay;

pub(crate) struct Delay {
    pub(crate) async_delay: AsyncDelay,
}

impl runtime_raw::Delay for Delay {}

impl Future for Delay {
    type Output = Instant;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // TODO: this should probably not be fallible.
        futures::ready!(Pin::new(&mut self.async_delay).poll(cx)).unwrap();
        Poll::Ready(Instant::now())
    }
}
