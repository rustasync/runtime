//! Types and Functions for time-related operations.
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
//! use runtime::time::Delay;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//! let now = Delay::new(Duration::from_secs(3)).await;
//!
//! let elapsed = now - start;
//! println!("elapsed: {}s", elapsed.as_secs());
//! # }
//! ```
//!
//! __Schedule a two-second interval__
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() {
//! # use futures::prelude::*;
//! use runtime::time::Interval;
//! use std::time::{Duration, Instant};
//!
//! let start = Instant::now();
//!
//! let mut interval = Interval::new(Duration::from_secs(2));
//! while let Some(now) = interval.next().await {
//!     let elapsed = now - start;
//!     println!("elapsed: {}s", elapsed.as_secs());
//! }
//! # }
//! ```

mod delay;
mod interval;
mod ext;

pub use delay::*;
pub use interval::*;
pub use ext::*;
