use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::compat::Compat01As03;
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
        let mut fut = Compat01As03::new(&mut self.tokio_delay);
        futures::ready!(Pin::new(&mut fut).poll(cx)).unwrap();
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
        let mut stream = Compat01As03::new(&mut self.tokio_interval);
        // https://docs.rs/tokio/0.1.20/tokio/timer/struct.Error.html
        futures::ready!(Pin::new(&mut stream).poll_next(cx))
            .unwrap()
            .unwrap();
        Poll::Ready(Some(Instant::now()))
    }
}
