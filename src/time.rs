//! Temporal manipulation.

mod delay;
mod interval;

pub mod ext;

pub use delay::Delay;
#[doc(inline)]
pub use ext::FutureExt;
pub use interval::Interval;

use std::time::{Duration, Instant};

/// Sleep the current future for the given duration.
#[inline]
pub fn wait_for(dur: Duration) -> Delay {
    Delay::new(dur)
}

/// Sleep the current future until the given time.
#[inline]
pub fn wait_until(at: Instant) -> Delay {
    Delay::new_at(at)
}

/// Create a stream that fires events at a set interval.
#[inline]
pub fn repeat(dur: Duration) -> Interval {
    Interval::new(dur)
}
