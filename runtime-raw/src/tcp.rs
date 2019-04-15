use futures::prelude::*;
use futures::task::{Poll, Waker};

use std::fmt::Debug;
use std::io;
use std::net::SocketAddr;

/// A TcpStream for this Runtime
pub trait TcpStream: AsyncRead + AsyncWrite + Debug + Send {
    /// Check if the stream can be written to.
    fn poll_write_ready(&self, waker: &Waker) -> Poll<io::Result<()>>;

    /// Check if the stream can be read from.
    fn poll_read_ready(&self, waker: &Waker) -> Poll<io::Result<()>>;

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
    /// Get the address the listener is listening on.
    fn local_addr(&self) -> io::Result<SocketAddr>;

    /// Check if the listener is ready to accept connections.
    fn poll_accept(&mut self, waker: &Waker) -> Poll<io::Result<Box<dyn TcpStream>>>;

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd;
}
