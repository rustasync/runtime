use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::Unimplemented;

impl runtime_raw::UdpSocket for Unimplemented {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!()
    }

    fn poll_send_to(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
        _receiver: &SocketAddr,
    ) -> Poll<io::Result<usize>> {
        unimplemented!()
    }

    fn poll_recv_from(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        unimplemented!()
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    fn broadcast(&self) -> io::Result<bool> {
        unimplemented!()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    fn set_broadcast(&self, _on: bool) -> io::Result<()> {
        unimplemented!()
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v4(&self) -> io::Result<bool> {
        unimplemented!()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v4(&self, _on: bool) -> io::Result<()> {
        unimplemented!()
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unimplemented!()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn set_multicast_ttl_v4(&self, _ttl: u32) -> io::Result<()> {
        unimplemented!()
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v6(&self) -> io::Result<bool> {
        unimplemented!()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v6(&self, _on: bool) -> io::Result<()> {
        unimplemented!()
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    fn ttl(&self) -> io::Result<u32> {
        unimplemented!()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    fn set_ttl(&self, _ttl: u32) -> io::Result<()> {
        unimplemented!()
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    fn join_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
        unimplemented!()
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    fn join_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
        unimplemented!()
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    fn leave_multicast_v4(&self, _multiaddr: &Ipv4Addr, _interface: &Ipv4Addr) -> io::Result<()> {
        unimplemented!()
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    fn leave_multicast_v6(&self, _multiaddr: &Ipv6Addr, _interface: u32) -> io::Result<()> {
        unimplemented!()
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        unimplemented!()
    }
}
