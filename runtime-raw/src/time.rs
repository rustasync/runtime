use std::fmt::Debug;
use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;
use std::time::Instant;

use futures::Stream;

/// A boxed type-erased [`Delay`].
pub type BoxDelay = Pin<Box<dyn Delay>>;

/// A boxed type-erased [`Interval`].
pub type BoxInterval = Pin<Box<dyn Interval>>;

/// A future representing the notification that an elapsed duration has occurred.
pub trait Delay: Future<Output = Instant> + Debug + Send {}

impl<P> Delay for Pin<P>
where
    P: DerefMut + Debug + Send + Unpin,
    P::Target: Delay,
{
}

/// A stream representing notifications at a fixed interval.
pub trait Interval: Stream<Item = Instant> + Debug + Send {}

impl<P> Interval for Pin<P>
where
    P: DerefMut + Debug + Send + Unpin,
    P::Target: Interval,
{
}
