use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::compat::Compat01As03;
use futures::prelude::*;
use futures::ready;

use crate::Compat;

impl runtime_raw::Delay for Compat<tokio::timer::Delay> {}

impl Future for Compat<tokio::timer::Delay> {
    type Output = Instant;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut fut = Compat01As03::new(self.get_mut().get_mut());
        ready!(Pin::new(&mut fut).poll(cx)).unwrap();
        Poll::Ready(Instant::now())
    }
}

impl runtime_raw::Interval for Compat<tokio::timer::Interval> {}

impl Stream for Compat<tokio::timer::Interval> {
    type Item = Instant;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut stream = Compat01As03::new(self.get_mut().get_mut());
        // https://docs.rs/tokio/0.1.20/tokio/timer/struct.Error.html
        ready!(Pin::new(&mut stream).poll_next(cx))
            .unwrap()
            .unwrap();
        Poll::Ready(Some(Instant::now()))
    }
}
