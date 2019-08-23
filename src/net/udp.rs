//! Asynchronous UDP bindings.
//!
//! To create a bi-directional UDP socket use [`UdpSocket::bind`]. Sending data from the socket is
//! done by using [`send_to`] which returns the [`SendTo`] future. Reading data from the socket is
//! done by using [`recv_from`] which returns the [`RecvFrom`] future.
//!
//! [`UdpSocket::bind`]: struct.UdpSocket.html#method.bind
//! [`send_to`]: struct.UdpSocket.html#method.send_to
//! [`recv_from`]: struct.UdpSocket.html#method.recv_from
//! [`RecvFrom`]: struct.RecvFrom.html
//! [`SendTo`]: struct.SendTo.html

use futures::prelude::*;

use async_datagram::AsyncDatagram;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A UDP socket.
///
/// After creating a `UdpSocket` by [`bind`]ing it to a socket address, data can be [sent to] and
/// [received from] any other socket address.
///
/// As stated in the User Datagram Protocol's specification in [IETF RFC 768], UDP is an unordered,
/// unreliable protocol. Refer to [`TcpListener`] and [`TcpStream`] for async TCP primitives, and
/// [`std::net`] for synchronous networking primitives.
///
/// [`bind`]: #method.bind
/// [received from]: #method.recv_from
/// [sent to]: #method.send_to
/// [`TcpListener`]: ../struct.TcpListener.html
/// [`TcpStream`]: ../struct.TcpStream.html
/// [`std::net`]: https://doc.rust-lang.org/std/net/index.html
/// [IETF RFC 768]: https://tools.ietf.org/html/rfc768
///
/// ## Examples
/// ```no_run
/// use runtime::net::UdpSocket;
///
/// #[runtime::main]
/// async fn main() -> std::io::Result<()> {
///     let mut socket = UdpSocket::bind("127.0.0.1:8080")?;
///     let mut buf = vec![0u8; 1024];
///
///     println!("Listening on {}", socket.local_addr()?);
///
///     loop {
///         let (recv, peer) = socket.recv_from(&mut buf).await?;
///         let sent = socket.send_to(&buf[..recv], &peer).await?;
///         println!("Sent {} out of {} bytes to {}", sent, recv, peer);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct UdpSocket {
    inner: Pin<Box<dyn runtime_raw::UdpSocket>>,
}

impl UdpSocket {
    /// Creates a UDP socket from the given address.
    ///
    /// Binding with a port number of 0 will request that the OS assigns a port to this socket. The
    /// port allocated can be queried via the [`local_addr`] method.
    ///
    /// [`local_addr`]: #method.local_addr
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use runtime::net::UdpSocket;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = UdpSocket::bind("127.0.0.1:0")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let mut last_err = None;
        for addr in addr.to_socket_addrs()? {
            match runtime_raw::current_runtime().bind_udp_socket(&addr) {
                Ok(inner) => return Ok(UdpSocket { inner }),
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
    /// This can be useful, for example, when binding to port 0 to figure out which port was
    /// actually bound.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///	use runtime::net::UdpSocket;
    ///
    ///	# fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let socket = UdpSocket::bind("127.0.0.1:0")?;
    /// println!("Address: {:?}", socket.local_addr());
    /// # Ok(())
    /// # }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    /// Sends data on the socket to the given address.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// use runtime::net::UdpSocket;
    ///
    /// const THE_MERCHANT_OF_VENICE: &[u8] = b"
    ///     If you prick us, do we not bleed?
    ///     If you tickle us, do we not laugh?
    ///     If you poison us, do we not die?
    ///     And if you wrong us, shall we not revenge?
    /// ";
    ///
    /// # async fn send_data() -> Result<(), Box<dyn Error + 'static>> {
    /// let mut socket = UdpSocket::bind("127.0.0.1:0")?;
    ///
    /// let addr = "127.0.0.1:7878";
    /// let sent = socket.send_to(THE_MERCHANT_OF_VENICE, &addr).await?;
    /// println!("Sent {} bytes to {}", sent, addr);
    /// # Ok(())
    /// # }
    /// ```
    pub fn send_to<'socket, 'buf, A: ToSocketAddrs>(
        &'socket mut self,
        buf: &'buf [u8],
        addr: A,
    ) -> SendToFuture<'socket, 'buf> {
        let addr = addr
            .to_socket_addrs()
            .map(|mut iter| iter.next())
            .transpose();
        SendToFuture {
            buf,
            addr,
            socket: self,
        }
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read and the origin.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// use runtime::net::UdpSocket;
    ///
    /// # async fn recv_data() -> Result<Vec<u8>, Box<dyn Error + 'static>> {
    /// let mut socket = UdpSocket::bind("127.0.0.1:0")?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let (recv, peer) = socket.recv_from(&mut buf).await?;
    /// println!("Received {} bytes from {}", recv, peer);
    /// # Ok(buf)
    /// # }
    /// ```
    pub fn recv_from<'socket, 'buf>(
        &'socket mut self,
        buf: &'buf mut [u8],
    ) -> RecvFromFuture<'socket, 'buf> {
        RecvFromFuture { buf, socket: self }
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// For more information about this option, see [`set_broadcast`].
    ///
    /// [`set_broadcast`]: #method.set_broadcast
    pub fn broadcast(&self) -> io::Result<bool> {
        self.inner.broadcast()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// When enabled, this socket is allowed to send packets to a broadcast
    /// address.
    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.inner.set_broadcast(on)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_loop_v4`].
    ///
    /// [`set_multicast_loop_v4`]: #method.set_multicast_loop_v4
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.inner.multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// If enabled, multicast packets will be looped back to the local socket.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv6 sockets.
    pub fn set_multicast_loop_v4(&self, on: bool) -> io::Result<()> {
        self.inner.set_multicast_loop_v4(on)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_ttl_v4`].
    ///
    /// [`set_multicast_ttl_v4`]: #method.set_multicast_ttl_v4
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.inner.multicast_ttl_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// Indicates the time-to-live value of outgoing multicast packets for
    /// this socket. The default value is 1 which means that multicast packets
    /// don't leave the local network unless explicitly requested.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv6 sockets.
    pub fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()> {
        self.inner.set_multicast_ttl_v4(ttl)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_loop_v6`].
    ///
    /// [`set_multicast_loop_v6`]: #method.set_multicast_loop_v6
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.inner.multicast_loop_v6()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// Controls whether this socket sees the multicast packets it sends itself.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv4 sockets.
    pub fn set_multicast_loop_v6(&self, on: bool) -> io::Result<()> {
        self.inner.set_multicast_loop_v6(on)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: #method.set_ttl
    pub fn ttl(&self) -> io::Result<u32> {
        self.inner.ttl()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.inner.set_ttl(ttl)
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This function specifies a new multicast group for this socket to join.  The address must be
    /// a valid multicast address, and `interface` is the address of the local interface with which
    /// the system should join the multicast group. If it's equal to `INADDR_ANY` then an
    /// appropriate interface is chosen by the system.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use runtime::net::UdpSocket;
    /// use std::net::Ipv4Addr;
    ///
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let interface = Ipv4Addr::new(0, 0, 0, 0);
    /// let mdns_addr = Ipv4Addr::new(224, 0, 0, 123);
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0")?;
    /// socket.join_multicast_v4(&mdns_addr, &interface)?;
    /// # Ok(()) }
    /// ```
    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.inner.join_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This function specifies a new multicast group for this socket to join.  The address must be
    /// a valid multicast address, and `interface` is the index of the interface to join/leave (or
    /// 0 to indicate any interface).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use runtime::net::UdpSocket;
    /// use std::net::{Ipv6Addr, SocketAddr};
    ///
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let socket_addr = SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), 0);
    /// let mdns_addr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123) ;
    /// let socket = UdpSocket::bind(&socket_addr)?;
    ///
    /// socket.join_multicast_v6(&mdns_addr, 0)?;
    /// # Ok(()) }
    /// ```
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.inner.join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v4`].
    ///
    /// [`join_multicast_v4`]: #method.join_multicast_v4
    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.inner.leave_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v6`].
    ///
    /// [`join_multicast_v6`]: #method.join_multicast_v6
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.inner.leave_multicast_v6(multiaddr, interface)
    }
}

impl AsyncDatagram for UdpSocket {
    type Sender = SocketAddr;
    type Receiver = SocketAddr;
    type Err = io::Error;

    fn poll_send_to(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &Self::Receiver,
    ) -> Poll<Result<usize, Self::Err>> {
        self.inner.as_mut().poll_send_to(cx, buf, receiver)
    }

    fn poll_recv_from(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<(usize, Self::Sender), Self::Err>> {
        self.inner.as_mut().poll_recv_from(cx, buf)
    }
}

/// The future returned by [`UdpSocket::send_to`].
///
/// On success, returns the number of bytes written.
///
/// [`UdpSocket::send_to`]: struct.UdpSocket.html#method.send_to
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct SendToFuture<'socket, 'buf> {
    /// The open socket we use to send the message from.
    socket: &'socket mut UdpSocket,
    /// The message we're trying to send.
    buf: &'buf [u8],
    /// The address we'll try to connect to.
    addr: Option<io::Result<SocketAddr>>,
}

impl<'socket, 'buf> Future for SendToFuture<'socket, 'buf> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let SendToFuture { socket, buf, addr } = &mut *self;
        let addr = match addr.take() {
            Some(addr) => addr?,
            None => {
                let err_msg = "no addresses to send data to";
                let err = io::Error::new(io::ErrorKind::InvalidInput, err_msg);
                return Poll::Ready(Err(err));
            }
        };
        let poll = socket.inner.as_mut().poll_send_to(cx, buf, &addr);
        self.addr = Some(Ok(addr));
        poll
    }
}

/// The future returned by [`UdpSocket::recv_from`].
///
/// On success, returns the number of bytes read and the origin.
///
/// [`UdpSocket::recv_from`]: struct.UdpSocket.html#method.recv_from
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct RecvFromFuture<'socket, 'buf> {
    socket: &'socket mut UdpSocket,
    buf: &'buf mut [u8],
}

impl<'socket, 'buf> Future for RecvFromFuture<'socket, 'buf> {
    type Output = io::Result<(usize, SocketAddr)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let RecvFromFuture { socket, buf } = &mut *self;
        socket.inner.as_mut().poll_recv_from(cx, buf)
    }
}

#[cfg(unix)]
mod sys {
    use super::UdpSocket;
    use std::os::unix::prelude::*;

    impl AsRawFd for UdpSocket {
        fn as_raw_fd(&self) -> RawFd {
            self.inner.as_raw_fd()
        }
    }
}
