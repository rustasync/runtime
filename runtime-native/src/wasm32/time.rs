use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use futures::{future::Future, stream::Stream};

use super::Unimplemented;

impl runtime_raw::Delay for Unimplemented {}

impl Future for Unimplemented {
    type Output = Instant;

    #[inline]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}

impl runtime_raw::Interval for Unimplemented {}

impl Stream for Unimplemented {
    type Item = Instant;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!()
    }
}
