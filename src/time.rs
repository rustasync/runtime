//! Types and Functions for temporal manipulation.
//!
//! This module provides primitives for setting asynchronous timeouts, intervals, and delays.
//!
//! # Organization
//!
//! * [`Delay`] and [`Interval`] provide functionality for setting delays and intervals.
//! * [`FutureExt`] extends Futures with the ability to time-out.
//! * Other types are return or parameter types for various methods in this module
//!
//! [`Delay`]: struct.Delay.html
//! [`Interval`]: struct.Interval.html
//! [`FutureExt`]: trait.FutureExt.html
//!
//! ## Examples
//! __Schedule a 3-second delay__
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() {
//! use runtime::time::wait_for;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//! let now = wait_for(Duration::from_secs(3)).await;
//!
//! let elapsed = now - start;
//! println!("elapsed: {}s", elapsed.as_secs());
//! # }
//! ```
//!
//! __Schedule a 2-second interval__
//! ```ignore
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() {
//! # use futures::for_await;
//! use runtime::time::repeat;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//!
//! #[for_await]
//! for now in repeat(Duration::from_secs(2)) {
//!     let elapsed = now - start;
//!     println!("elapsed: {}s", elapsed.as_secs());
//! }
//! # }
//! ```

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
