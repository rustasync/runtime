use futures::prelude::*;
use futures::task::{Context, Poll};

use std::fmt::Debug;
use std::io;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::pin::Pin;

/// A boxed type-erased [`TcpStream`].
pub type BoxTcpStream = Pin<Box<dyn TcpStream>>;

/// A boxed type-erased [`TcpListener`] that returns boxed type-erased streams.
pub type BoxTcpListener = Pin<Box<dyn TcpListener<TcpStream = BoxTcpStream>>>;

/// A TcpStream for this Runtime
pub trait TcpStream: AsyncRead + AsyncWrite + Debug + Send {
    /// Check if the stream can be written to.
    fn poll_write_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

    /// Check if the stream can be read from.
    fn poll_read_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>>;

    /// Check if any socket errors exist on the `TcpStream`.
    ///
    /// Checking for socket errors is fallible, which is why the outer type is
    /// `Result`.
    fn take_error(&self) -> io::Result<Option<io::Error>>;

    /// Returns the local address that this stream is connected to.
    fn local_addr(&self) -> io::Result<SocketAddr>;

    /// Returns the remote address that this stream is connected to.
    fn peer_addr(&self) -> io::Result<SocketAddr>;

    /// Shuts down the read, write, or both halves of this connection.
    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()>;

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd;
}

/// A TcpListener for this Runtime
pub trait TcpListener: Debug + Send {
    /// The [`TcpStream`] implementation for this [`TcpListener`].
    type TcpStream: TcpStream + 'static;

    /// Get the address the listener is listening on.
    fn local_addr(&self) -> io::Result<SocketAddr>;

    /// Check if the listener is ready to accept connections.
    fn poll_accept(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<io::Result<Self::TcpStream>>;

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd;
}

impl<P> TcpStream for Pin<P>
where
    P: DerefMut + Debug + Send + Unpin,
    P::Target: TcpStream,
{
    fn poll_write_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().as_mut().poll_write_ready(cx)
    }

    fn poll_read_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().as_mut().poll_read_ready(cx)
    }

    fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.as_ref().take_error()
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.as_ref().local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.as_ref().peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.as_ref().shutdown(how)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.as_ref().as_raw_fd()
    }
}

impl<P> TcpListener for Pin<P>
where
    P: DerefMut + Debug + Send + Unpin,
    P::Target: TcpListener,
{
    type TcpStream = <P::Target as TcpListener>::TcpStream;

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.as_ref().local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Self::TcpStream>> {
        self.get_mut().as_mut().poll_accept(cx)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.as_ref().as_raw_fd()
    }
}
