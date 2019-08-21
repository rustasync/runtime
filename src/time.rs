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
//! __Delay execution for three seconds__
//! ```no_run
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
//! __Emit an event every two seconds__
//! ```no_run
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
mod ext;
mod interval;

pub use delay::*;
pub use ext::*;
pub use interval::*;
