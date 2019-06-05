//! Extensions for Futures types.

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::{AsyncRead, Stream};

use super::Delay;

/// A future returned by methods in the [`FutureExt`] trait.
///
/// [`FutureExt.timeout`]: trait.FutureExt.html
#[derive(Debug)]
pub struct Timeout<F: Future> {
    future: F,
    delay: Delay,
}

impl<F: Future> Timeout<F> {
    pin_utils::unsafe_pinned!(future: F);
    pin_utils::unsafe_pinned!(delay: Delay);
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.as_mut().future().poll(cx) {
            Poll::Pending => {}
            Poll::Ready(t) => return Poll::Ready(Ok(t)),
        }

        if self.as_mut().poll(cx).is_ready() {
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out").into());
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}

/// Extend `Future` with methods to time out execution.
pub trait FutureExt: Future + Sized {
    /// Creates a new future which will take at most `dur` time to resolve from
    /// the point at which this method is called.
    ///
    /// This combinator creates a new future which wraps the receiving future
    /// in a timeout. The future returned will resolve in at most `dur` time
    /// specified (relative to when this function is called).
    ///
    /// If the future completes before `dur` elapses then the future will
    /// resolve with that item. Otherwise the future will resolve to an error
    /// once `dur` has elapsed.
    ///
    /// # Examples
    /// ```
    /// # #![feature(async_await)]
    /// use futures::prelude::*;
    /// use runtime::prelude::*;
    /// use std::time::Duration;
    ///
    /// # fn long_future() -> impl Future<Output = std::io::Result<()>> {
    /// #     futures::future::ok(())
    /// # }
    /// #
    /// #[runtime::main]
    /// async fn main() {
    ///     let future = long_future();
    ///     let timed_out = future.timeout(Duration::from_millis(100));
    ///
    ///     match timed_out.await {
    ///         Ok(item) => println!("got {:?} within enough time!", item),
    ///         Err(_) => println!("took too long to produce the item"),
    ///     }
    /// }
    /// ```
    fn timeout(self, dur: Duration) -> Timeout<Self> {
        Timeout {
            delay: Delay::new(dur),
            future: self,
        }
    }

    /// Creates a new future which will resolve no later than `at` specified.
    ///
    /// This method is otherwise equivalent to the [`timeout`] method except that
    /// it tweaks the moment at when the timeout elapsed to being specified with
    /// an absolute value rather than a relative one. For more documentation see
    /// the [`timeout`] method.
    ///
    /// [`timeout`]: trait.FutureExt.html#method.timeout
    ///
    /// # Examples
    /// ```
    /// # #![feature(async_await)]
    /// use futures::prelude::*;
    /// use runtime::prelude::*;
    /// use std::time::{Duration, Instant};
    ///
    /// # fn long_future() -> impl Future<Output = std::io::Result<()>> {
    /// #     futures::future::ok(())
    /// # }
    /// #
    /// #[runtime::main]
    /// async fn main() {
    ///     let future = long_future();
    ///     let at = Instant::now() + Duration::from_millis(100);
    ///     let timed_out = future.timeout_at(at);
    ///
    ///     match timed_out.await {
    ///         Ok(item) => println!("got {:?} within enough time!", item),
    ///         Err(_) => println!("took too long to produce the item"),
    ///     }
    /// }
    /// ```
    fn timeout_at(self, at: Instant) -> Timeout<Self> {
        Timeout {
            delay: Delay::new_at(at),
            future: self,
        }
    }
}

impl<T: Future> FutureExt for T {}

/// A stream returned by methods in the [`StreamExt`] trait.
///
/// [`StreamExt`]: trait.StreamExt.html
#[derive(Debug)]
pub struct TimeoutStream<S: Stream> {
    timeout: Delay,
    dur: Duration,
    stream: S,
}

impl<S: Stream> TimeoutStream<S> {
    pin_utils::unsafe_pinned!(timeout: Delay);
    pin_utils::unsafe_pinned!(stream: S);
}

impl<S: Stream> Stream for TimeoutStream<S> {
    type Item = Result<S::Item, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.as_mut().stream().poll_next(cx) {
            Poll::Pending => {}
            Poll::Ready(s) => {
                *self.as_mut().timeout() = Delay::new(self.dur);
                let res = Ok(s).transpose();
                return Poll::Ready(res);
            }
        }

        if self.as_mut().timeout().poll(cx).is_ready() {
            *self.as_mut().timeout() = Delay::new(self.dur);
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out").into());
            Poll::Ready(Some(err))
        } else {
            Poll::Pending
        }
    }
}

/// Extend `Stream` with methods to time out execution.
pub trait StreamExt: Stream + Sized {
    /// Creates a new stream which will take at most `dur` time to yield each
    /// item of the stream.
    ///
    /// This combinator creates a new stream which wraps the receiving stream
    /// in a timeout-per-item. The stream returned will resolve in at most
    /// `dur` time for each item yielded from the stream. The first item's timer
    /// starts when this method is called.
    ///
    /// If a stream's item completes before `dur` elapses then the timer will be
    /// reset for the next item. If the timeout elapses, however, then an error
    /// will be yielded on the stream and the timer will be reset.
    ///
    /// ## Examples
    /// ```
    /// # #![feature(async_await)]
    /// # use futures::prelude::*;
    /// use runtime::time::{Interval, StreamExt as _};
    /// use std::time::{Duration, Instant};
    ///
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let start = Instant::now();
    /// let mut interval = Interval::new(Duration::from_millis(10))
    ///     .take(3)
    ///     .timeout(Duration::from_millis(15));
    /// while let Some(now) = interval.next().await {
    ///     println!("{}ms have elapsed", (now? - start).as_millis());
    /// }
    ///
    /// assert!(Instant::now() - start >= Duration::from_millis(30));
    /// # Ok(())}
    /// ```
    fn timeout(self, dur: Duration) -> TimeoutStream<Self> {
        TimeoutStream {
            timeout: Delay::new(dur),
            dur,
            stream: self,
        }
    }
}

impl<S: Stream> StreamExt for S {}

/// A stream returned by methods in the [`StreamExt`] trait.
///
/// [`StreamExt`]: trait.StreamExt.html
#[derive(Debug)]
pub struct TimeoutAsyncRead<S: AsyncRead> {
    timeout: Delay,
    dur: Duration,
    stream: S,
}

impl<S: AsyncRead> TimeoutAsyncRead<S> {
    pin_utils::unsafe_pinned!(timeout: Delay);
    pin_utils::unsafe_pinned!(stream: S);
}

impl<S: AsyncRead> AsyncRead for TimeoutAsyncRead<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        match self.as_mut().stream().poll_read(cx, buf) {
            Poll::Pending => {}
            Poll::Ready(s) => {
                *self.as_mut().timeout() = Delay::new(self.dur);
                return Poll::Ready(s);
            }
        }

        if self.as_mut().timeout().poll(cx).is_ready() {
            *self.as_mut().timeout() = Delay::new(self.dur);
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out").into());
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}

/// Extend `AsyncRead` with methods to time out execution.
pub trait AsyncReadExt: AsyncRead + Sized {
    /// Creates a new stream which will take at most `dur` time to yield each
    /// item of the stream.
    ///
    /// This combinator creates a new stream which wraps the receiving stream
    /// in a timeout-per-item. The stream returned will resolve in at most
    /// `dur` time for each item yielded from the stream. The first item's timer
    /// starts when this method is called.
    ///
    /// If a stream's item completes before `dur` elapses then the timer will be
    /// reset for the next item. If the timeout elapses, however, then an error
    /// will be yielded on the stream and the timer will be reset.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + 'static + Send + Sync>> {
    /// use futures::prelude::*;
    /// use runtime::prelude::*;
    /// use runtime::net::TcpStream;
    /// use std::time::{Duration, Instant};
    ///
    /// let start = Instant::now();
    ///
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    /// let _stream = stream.timeout(Duration::from_millis(100));
    /// # Ok(())}
    /// ```
    fn timeout(self, dur: Duration) -> TimeoutAsyncRead<Self> {
        TimeoutAsyncRead {
            timeout: Delay::new(dur),
            dur,
            stream: self,
        }
    }
}

impl<S: AsyncRead> AsyncReadExt for S {}
