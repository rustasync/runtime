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
//! __Schedule a three-second delay__
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() {
//! use runtime::time::delay_for;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//! let now = delay_for(Duration::from_secs(3)).await;
//!
//! let elapsed = now - start;
//! println!("elapsed: {}s", elapsed.as_secs());
//! # }
//! ```
//!
//! __Schedule a two-second interval__
//! ```ignore
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() {
//! # use futures::for_await;
//! use runtime::time::interval;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//!
//! #[for_await]
//! for now in interval(Duration::from_secs(2)) {
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
///
/// ## Examples
/// ```
/// # #![feature(async_await)]
/// use runtime::time::delay_for;
/// use std::time::{Duration, Instant};
///
/// # #[runtime::main]
/// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let start = Instant::now();
/// let now = delay_for(Duration::from_millis(20)).await;
///
/// assert!(now - start >= Duration::from_millis(20));
/// # Ok(())}
/// ```
#[inline]
pub fn delay_for(dur: Duration) -> Delay {
    Delay::new(dur)
}

/// Sleep the current future until the given time.
///
/// ## Examples
/// ```
/// # #![feature(async_await)]
/// use runtime::time::delay_until;
/// use std::time::{Duration, Instant};
///
/// # #[runtime::main]
/// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let start = Instant::now();
/// let now = delay_until(start + Duration::from_millis(40)).await;
///
/// assert!(now - start >= Duration::from_millis(40));
/// # Ok(())}
/// ```
#[inline]
pub fn delay_until(at: Instant) -> Delay {
    Delay::new_at(at)
}

/// Create a stream that fires events at a set interval.
///
/// ## Examples
/// ```
/// # #![feature(async_await)]
/// # use futures::prelude::*;
/// use runtime::time::interval;
/// use std::time::{Duration, Instant};
///
/// # #[runtime::main]
/// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let start = Instant::now();
/// let mut interval = interval(Duration::from_millis(10)).take(3);
/// while let Some(now) = interval.next().await {
///     println!("{}ms have elapsed", (now - start).as_millis());
/// }
///
/// assert!(Instant::now() - start >= Duration::from_millis(30));
/// # Ok(())}
/// ```
#[inline]
pub fn interval(dur: Duration) -> Interval {
    Interval::new(dur)
}
