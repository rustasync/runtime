//! Extensions for Futures types.

use std::future::Future;
use std::io;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::{AsyncRead, Stream};

use super::Delay;

/// A future returned by methods in the [`FutureExt`] trait.
///
/// [`FutureExt.timeout`]: trait.FutureExt.html
#[derive(Debug)]
pub struct Timeout<F: Future + Unpin> {
    future: F,
    delay: Delay,
}

impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Result<F::Output, io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(t) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Ok(t));
        }

        self.as_mut()
            .poll(cx)
            .map(|_| Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out")))
    }
}

/// Extend `Future` with methods to time out execution.
pub trait FutureExt: Future + Sized + Unpin {
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

impl<T: Future + Unpin> FutureExt for T {}

/// A stream returned by methods in the [`StreamExt`] trait.
///
/// [`StreamExt`]: trait.StreamExt.html
#[derive(Debug)]
pub struct TimeoutStream<S: Stream + Unpin> {
    timeout: Delay,
    dur: Duration,
    stream: S,
}

impl<S: Stream + Unpin> Stream for TimeoutStream<S> {
    type Item = Result<S::Item, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(s) = Pin::new(&mut self.stream).poll_next(cx) {
            self.timeout = Delay::new(self.dur);
            return Poll::Ready(Ok(s).transpose());
        }

        Pin::new(&mut self.timeout).poll(cx).map(|_| {
            self.timeout = Delay::new(self.dur);
            Some(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "future timed out",
            )))
        })
    }
}

/// Extend `Stream` with methods to time out execution.
pub trait StreamExt: Stream + Sized + Unpin {
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

impl<S: Stream + Unpin> StreamExt for S {}

/// A stream returned by methods in the [`StreamExt`] trait.
///
/// [`StreamExt`]: trait.StreamExt.html
#[derive(Debug)]
pub struct TimeoutAsyncRead<S: AsyncRead + Unpin> {
    timeout: Delay,
    dur: Duration,
    stream: S,
}

impl<S: AsyncRead + Unpin> AsyncRead for TimeoutAsyncRead<S> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        if let Poll::Ready(s) = Pin::new(&mut self.stream).poll_read(cx, buf) {
            self.timeout = Delay::new(self.dur);
            return Poll::Ready(s);
        }

        Pin::new(&mut self.timeout).poll(cx).map(|_| {
            self.timeout = Delay::new(self.dur);
            Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out"))
        })
    }
}

/// Extend `AsyncRead` with methods to time out execution.
pub trait AsyncReadExt: AsyncRead + Sized + Unpin {
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

impl<S: AsyncRead + Unpin> AsyncReadExt for S {}
