use std::fmt::Debug;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A UDP socket.
pub trait UdpSocket: Debug + Send + Sync {
    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, when binding to port 0 to figure out
    /// which port was actually bound.
    fn local_addr(&self) -> io::Result<SocketAddr>;

    /// Sends data on the IO interface to the specified target.
    ///
    /// On success, returns the number of bytes written.
    fn poll_send_to(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &SocketAddr,
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

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    fn broadcast(&self) -> io::Result<bool>;

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    fn set_broadcast(&self, on: bool) -> io::Result<()>;

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v4(&self) -> io::Result<bool>;

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v4(&self, on: bool) -> io::Result<()>;

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn multicast_ttl_v4(&self) -> io::Result<u32>;

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()>;

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v6(&self) -> io::Result<bool>;

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v6(&self, on: bool) -> io::Result<()>;

    /// Gets the value of the `IP_TTL` option for this socket.
    fn ttl(&self) -> io::Result<u32>;

    /// Sets the value for the `IP_TTL` option on this socket.
    fn set_ttl(&self, ttl: u32) -> io::Result<()>;

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()>;

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()>;

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()>;

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()>;

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd;
}
