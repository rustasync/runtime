//! Asynchronous TCP bindings.
//!
//! Connecting to an address over TCP is done by using [`TcpStream::connect`]. This returns a
//! [`Connect`] future which resolves to a [`TcpStream`].
//!
//! To listen for incoming TCP connections use [`TcpListener.bind`], which creates a new
//! [`TcpListener`]. Then use the the [`incoming`] method to accept new connections, yielding a
//! stream of [`TcpStream`]s.
//!
//! [`TcpStream`]: struct.TcpStream.html
//! [`TcpStream::connect`]: struct.TcpStream.html#method.connect
//! [`Connect`]: struct.Connect.html
//! [`TcpListener`]: struct.TcpListener.html
//! [`TcpListener.bind`]: struct.TcpListener.html#method.bind
//! [`incoming`]: struct.TcpListener.html#method.incoming
//! [`Incoming`]: struct.Incoming.html

use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::pin::Pin;

use futures::io::*;
use futures::prelude::*;
use futures::ready;
use futures::task::{Context, Poll};

/// A TCP stream between a local and a remote socket.
///
/// A `TcpStream` can either be created by connecting to an endpoint, via the [`connect`] method,
/// or by [accepting] a connection from a [listener].  It can be read or written to using the
/// [`AsyncRead`], [`AsyncWrite`], and related extension traits in [`futures::io`].
///
/// The connection will be closed when the value is dropped. The reading and writing portions of
/// the connection can also be shut down individually with the [`shutdown`] method.
///
/// [`connect`]: struct.TcpStream.html#method.connect
/// [accepting]: struct.TcpListener.html#method.accept
/// [listener]: struct.TcpListener.html
/// [`AsyncRead`]: https://docs.rs/futures-preview/0.3.0-alpha.13/futures/io/trait.AsyncRead.html
/// [`AsyncWrite`]: https://docs.rs/futures-preview/0.3.0-alpha.13/futures/io/trait.AsyncRead.html
/// [`futures::io`]: https://docs.rs/futures-preview/0.3.0-alpha.13/futures/io
/// [`shutdown`]: struct.TcpStream.html#method.shutdown
///
/// ## Examples
/// ```no_run
/// #![feature(async_await, await_macro, futures_api)]
///
/// use futures::prelude::*;
/// use runtime::net::TcpStream;
///
/// #[runtime::main]
/// async fn main() -> Result<(), failure::Error> {
///     let mut stream = await!(TcpStream::connect("127.0.0.1:8080"))?;
///     println!("Connected to {}", &stream.peer_addr()?);
///
///     let msg = "hello world";
///     println!("<- {}", msg);
///     await!(stream.write_all(msg.as_bytes()))?;
///
///     let mut buf = vec![0u8; 1024];
///     await!(stream.read(&mut buf))?;
///     println!("-> {}\n", std::str::from_utf8(&mut buf)?);
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct TcpStream {
    inner: Pin<Box<dyn runtime_raw::TcpStream>>,
}

impl TcpStream {
    /// Create a new TCP stream connected to the specified address.
    ///
    /// This function will create a new TCP socket and attempt to connect it to
    /// the `addr` provided. The [returned future] will be resolved once the
    /// stream has successfully connected, or it will return an error if one
    /// occurs.
    ///
    /// [returned future]: struct.Connect.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    /// use runtime::net::TcpStream;
    ///
    /// # async fn connect_localhost() -> std::io::Result<()> {
    /// let stream = await!(TcpStream::connect("127.0.0.1:0"))?;
    /// # Ok(())}
    /// ```
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Connect {
        Connect {
            addrs: Some(addr.to_socket_addrs().map(|iter| iter.collect())),
            last_err: None,
            future: None,
            runtime: runtime_raw::current_runtime(),
        }
    }

    /// Returns the local address that this stream is connected to.
    ///
    /// ## Examples
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    /// use runtime::net::TcpStream;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// # #[runtime::main]
    /// # async fn main() -> std::io::Result<()> {
    /// let stream = await!(TcpStream::connect("127.0.0.1:8080"))?;
    ///
    /// let expected = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    /// assert_eq!(stream.local_addr()?.ip(), expected);
    /// # Ok(())}
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    /// Returns the remote address that this stream is connected to.
    ///
    /// ## Examples
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    /// use runtime::net::TcpStream;
    /// use std::net::{IpAddr, Ipv4Addr};
    ///
    /// # async fn connect_localhost() -> std::io::Result<()> {
    /// let stream = await!(TcpStream::connect("127.0.0.1:8080"))?;
    ///
    /// let expected = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    /// assert_eq!(stream.peer_addr()?.ip(), expected);
    /// # Ok(())}
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.peer_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O on the specified
    /// portions to return immediately with an appropriate value (see the
    /// documentation of [`Shutdown`]).
    ///
    /// [`Shutdown`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    ///
    /// use std::net::Shutdown;
    /// use runtime::net::TcpStream;
    ///
    /// # #[runtime::main]
    /// # async fn main() -> std::io::Result<()> {
    /// let stream = await!(TcpStream::connect("127.0.0.1:8080"))?;
    /// stream.shutdown(Shutdown::Both)?;
    /// # Ok(()) }
    /// ```
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.inner.shutdown(how)
    }
}

impl AsyncRead for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.inner.poll_read(cx, buf)
    }

    fn poll_vectored_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        vec: &mut [&mut IoVec],
    ) -> Poll<io::Result<usize>> {
        self.inner.poll_vectored_read(cx, vec)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.inner.poll_close(cx)
    }

    fn poll_vectored_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        vec: &[&IoVec],
    ) -> Poll<io::Result<usize>> {
        self.inner.poll_vectored_write(cx, vec)
    }
}

/// The future returned by [`TcpStream::connect`].
///
/// Resolves to a [`TcpStream`] when the stream is connected.
///
/// [`TcpStream::connect`]: struct.TcpStream.html#method.connect
/// [`TcpStream`]: struct.TcpStream.html
pub struct Connect {
    addrs: Option<io::Result<VecDeque<SocketAddr>>>,
    last_err: Option<io::Error>,
    future:
        Option<Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> + Send>>>,
    runtime: &'static dyn runtime_raw::Runtime,
}

impl Future for Connect {
    type Output = io::Result<TcpStream>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            // Poll the connect future, if there is one.
            if let Some(future) = self.future.as_mut() {
                match future.as_mut().poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Ok(inner)) => return Poll::Ready(Ok(TcpStream { inner })),
                    Poll::Ready(Err(err)) => self.last_err = Some(err),
                }
            }

            // Get the list of addresses, or return an error if the list couldn't be parsed.
            let addrs = match self.addrs.as_mut().expect("polled a completed future") {
                Ok(addrs) => addrs,
                Err(_) => {
                    return Poll::Ready(Err(self.addrs.take().unwrap().err().unwrap()));
                }
            };

            // Get the next address from the list, or return an error if the list is empty.
            let addr = match addrs.pop_front() {
                Some(addr) => addr,
                None => {
                    let err = self.last_err.take().unwrap_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "could not resolve to any addresses",
                        )
                    });
                    return Poll::Ready(Err(err));
                }
            };

            // Initialize the next connect future.
            self.future = Some(self.runtime.connect_tcp_stream(&addr));
        }
    }
}

impl fmt::Debug for Connect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connect")
            .field("addrs", &self.addrs)
            .finish()
    }
}

/// A TCP socket server, listening for connections.
///
/// After creating a `TcpListener` by [`bind`]ing it to a socket address, it listens for incoming
/// TCP connections. These can be accepted by awaiting elements from the async stream of incoming
/// connections, [`incoming`][`TcpListener::incoming`].
///
/// The socket will be closed when the value is dropped.
///
/// The Transmission Control Protocol is specified in [IETF RFC 793].
///
/// [`bind`]: #method.bind
/// [`TcpListener::incoming`]: #method.incoming
/// [IETF RFC 793]: https://tools.ietf.org/html/rfc793
///
/// # Examples
/// ```ignore
/// #![feature(async_await, await_macro, futures_api)]
///
/// use futures::prelude::*;
/// use runtime::net::TcpListener;
///
/// #[runtime::main]
/// async fn main() -> std::io::Result<()> {
///     let mut listener = TcpListener::bind("127.0.0.1:8080")?;
///     println!("Listening on {}", listener.local_addr()?);
///
///     // accept connections and process them in parallel
///     let mut incoming = listener.incoming();
///     while let Some(stream) = await!(incoming.next()) {
///         runtime::spawn(async move {
///             let stream = stream?;
///             println!("Accepting from: {}", stream.peer_addr()?);
///
///             let (reader, writer) = &mut stream.split();
///             await!(reader.copy_into(writer))?;
///             Ok::<(), std::io::Error>(())
///         });
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct TcpListener {
    inner: Pin<Box<dyn runtime_raw::TcpListener>>,
}

impl TcpListener {
    /// Creates a new `TcpListener` which will be bound to the specified
    /// address.
    ///
    /// The returned listener is ready for accepting connections.
    ///
    /// Binding with a port number of 0 will request that the OS assigns a port
    /// to this listener. The port allocated can be queried via the
    /// [`local_addr`] method.
    ///
    /// # Examples
    /// Create a TCP listener bound to 127.0.0.1:0:
    ///
    /// ```no_run
    /// use runtime::net::TcpListener;
    ///
    /// # fn main () -> Result<(), Box<dyn std::error::Error + 'static>> {
    /// let listener = TcpListener::bind("127.0.0.1:0")?;
    /// # Ok(())}
    /// ```
    ///
    /// [`local_addr`]: #method.local_addr
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            match runtime_raw::current_runtime().bind_tcp_listener(&addr) {
                Ok(inner) => return Ok(TcpListener { inner }),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any addresses",
            )
        }))
    }

    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, to identify when binding to port 0
    /// which port was assigned by the OS.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    ///
    /// use runtime::net::TcpListener;
    /// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    ///
    /// # #[runtime::main]
    /// # async fn main () -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let listener = TcpListener::bind("127.0.0.1:8080")?;
    ///
    /// let expected = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// assert_eq!(listener.local_addr()?, SocketAddr::V4(expected));
    /// # Ok(())}
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    /// Handle all incoming connections.
    ///
    /// This method returns a stream of [`TcpStream`]s. This is useful when you
    /// want to open up a port that can handle multiple incoming requests.
    ///
    /// If you intend to only handle single connections use [`.accept()`].
    ///
    /// [`TcpStream`]: struct.TcpStream.html
    /// [`.accept()`]: struct.TcpListener.html#method.accept
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    ///
    /// use futures::prelude::*;
    /// use runtime::net::TcpListener;
    ///
    /// # async fn work () -> Result<(), Box<dyn std::error::Error + 'static>> {
    /// let mut listener = TcpListener::bind("127.0.0.1:0")?;
    /// let mut incoming = listener.incoming();
    /// while let Some(stream) = await!(incoming.next()) {
    ///     match stream {
    ///         Ok(stream) => println!("new client!"),
    ///         Err(e) => { /* connection failed */ }
    ///     }
    /// }
    /// # Ok(())}
    /// ```
    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming { inner: self }
    }

    /// Handle an incoming connection.
    ///
    /// This is useful when you quickly want to receive an incoming TCP
    /// connection to quickly connect two points on a network.
    ///
    /// If you intend to handle all incoming connections use [`.incoming()`].
    ///
    /// [`TcpStream`]: struct.TcpStream.html
    /// [`.incoming()`]: struct.TcpListener.html#method.incoming
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// #![feature(async_await, await_macro, futures_api)]
    ///
    /// use futures::prelude::*;
    /// use runtime::net::TcpListener;
    ///
    /// # async fn work () -> Result<(), Box<dyn std::error::Error + 'static>> {
    /// let mut listener = TcpListener::bind("127.0.0.1:0")?;
    /// let (stream, addr) = await!(listener.accept())?;
    /// println!("Connected to {}", addr);
    /// # Ok(())}
    /// ```
    pub fn accept(&mut self) -> Accept<'_> {
        let incoming = self.incoming();
        Accept { inner: incoming }
    }
}

/// The future returned by [`TcpStream::accept`].
///
/// Resolves to a [`TcpStream`] when the future resolves.
///
/// [`TcpStream::accept`]: struct.TcpStream.html#method.accept
/// [`TcpStream`]: struct.TcpStream.html
#[derive(Debug)]
pub struct Accept<'stream> {
    inner: Incoming<'stream>,
}

impl<'stream> Future for Accept<'stream> {
    type Output = io::Result<(TcpStream, SocketAddr)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match ready!(self.inner.poll_next_unpin(cx)).unwrap() {
            Err(err) => Poll::Ready(Err(err)),
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                Poll::Ready(Ok((stream, addr)))
            }
        }
    }
}

/// A stream that infinitely [`accept`]s connections on a [`TcpListener`].
///
/// This `struct` is created by the [`incoming`] method on [`TcpListener`].
/// See its documentation for more.
///
/// [`incoming`]: struct.TcpListener.html#method.incoming
/// [`accept`]: struct.TcpStream.html#method.accept
/// [`TcpListener`]: struct.TcpStream.html
#[derive(Debug)]
pub struct Incoming<'listener> {
    inner: &'listener mut TcpListener,
}

impl<'listener> Stream for Incoming<'listener> {
    type Item = io::Result<TcpStream>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let inner = ready!(self.inner.inner.poll_accept(cx)?);
        Poll::Ready(Some(Ok(TcpStream { inner })))
    }
}

#[cfg(unix)]
mod sys {
    use super::{TcpListener, TcpStream};
    use std::os::unix::prelude::*;

    impl AsRawFd for TcpListener {
        fn as_raw_fd(&self) -> RawFd {
            self.inner.as_raw_fd()
        }
    }

    impl AsRawFd for TcpStream {
        fn as_raw_fd(&self) -> RawFd {
            self.inner.as_raw_fd()
        }
    }
}
