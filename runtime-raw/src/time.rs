use std::fmt::Debug;
use std::future::Future;
use std::time::Instant;

use futures::Stream;

/// A future representing the notification that an elapsed duration has occurred.
pub trait Delay: Future<Output = Instant> + Debug + Send {}

/// A stream representing notifications at a fixed interval.
pub trait Interval: Stream<Item = Instant> + Debug + Send {}
