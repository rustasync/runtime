use async_datagram::AsyncDatagram;

use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::task::{Poll, Waker};

#[derive(Debug)]
pub(crate) struct UdpSocket {
    pub romio_socket: romio::udp::UdpSocket,
}

impl runtime_raw::UdpSocket for UdpSocket {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_socket.local_addr()
    }

    fn poll_send_to(
        &mut self,
        waker: &Waker,
        buf: &[u8],
        receiver: &SocketAddr,
    ) -> Poll<io::Result<usize>> {
        self.romio_socket.poll_send_to(waker, buf, receiver)
    }

    fn poll_recv_from(
        &mut self,
        waker: &Waker,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        self.romio_socket.poll_recv_from(waker, buf)
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    fn broadcast(&self) -> io::Result<bool> {
        self.romio_socket.broadcast()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.romio_socket.set_broadcast(on)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.romio_socket.multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v4(&self, on: bool) -> io::Result<()> {
        self.romio_socket.set_multicast_loop_v4(on)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.romio_socket.multicast_ttl_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()> {
        self.romio_socket.set_multicast_ttl_v4(ttl)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.romio_socket.multicast_loop_v6()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    fn set_multicast_loop_v6(&self, on: bool) -> io::Result<()> {
        self.romio_socket.set_multicast_loop_v6(on)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    fn ttl(&self) -> io::Result<u32> {
        self.romio_socket.ttl()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.romio_socket.set_ttl(ttl)
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.romio_socket.join_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.romio_socket.join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.romio_socket.leave_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.romio_socket.leave_multicast_v6(multiaddr, interface)
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.romio_socket.as_raw_fd()
    }
}
