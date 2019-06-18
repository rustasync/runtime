use std::fmt::Debug;
use std::io;
use std::net::Shutdown;
use std::os::unix::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A UDP socket.
pub trait UnixDatagram: Debug + Send {
    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, when binding to port 0 to figure out
    /// which port was actually bound.
    fn local_addr(&self) -> io::Result<SocketAddr>;

    /// Returns the address of this socket's peer.
    ///
    /// The `bind` method will connect the socket to a peer.
    fn peer_addr(&self) -> io::Result<SocketAddr>;

    /// Sends data on the IO interface to the specified target.
    ///
    /// On success, returns the number of bytes written.
    fn poll_send_to(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &PathBuf,
    ) -> Poll<io::Result<usize>>;

    /// Receives data from the IO interface.
    ///
    /// On success, returns the number of bytes read and the target from whence
    /// the data came.
    fn poll_recv_from(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>>;

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the
    /// specified portions to immediately return with an appropriate value
    /// (see the documentation of `Shutdown`).
    fn shutdown(&self, how: Shutdown) -> io::Result<()>;
}
