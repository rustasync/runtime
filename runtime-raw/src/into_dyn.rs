use crate::{
    BoxDelay, BoxInterval, BoxTcpListener, BoxTcpStream, BoxUdpSocket, Runtime, TcpListener,
};
use futures::{future::BoxFuture, ready, task::SpawnError};
use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

/// Maps all the associated types on [`Runtime`] related traits into boxed trait objects to fully
/// type erase a `Runtime`.
#[derive(Debug)]
pub(crate) struct Dyn<T>(T);

impl<T> Dyn<T> {
    /// Create a new [`Dyn`].
    pub const fn new(t: T) -> Self {
        Self(t)
    }

    /// Project into a pinned [`Dyn`].
    #[allow(unsafe_code)]
    pub fn get_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T> {
        unsafe { Pin::new_unchecked(&mut Pin::get_unchecked_mut(self).0) }
    }
}

impl<R: Runtime> Runtime for Dyn<R> {
    type TcpStream = BoxTcpStream;
    type TcpListener = BoxTcpListener;
    type UdpSocket = BoxUdpSocket;
    type Delay = BoxDelay;
    type Interval = BoxInterval;
    type ConnectTcpStream = BoxFuture<'static, io::Result<BoxTcpStream>>;

    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        self.0.spawn_boxed(fut)
    }

    fn connect_tcp_stream(&self, addr: &SocketAddr) -> Self::ConnectTcpStream {
        let fut = self.0.connect_tcp_stream(addr);
        Box::pin(async { Ok(Box::pin(fut.await?) as _) })
    }

    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Self::TcpListener> {
        Ok(Box::pin(Dyn::new(self.0.bind_tcp_listener(addr)?)))
    }

    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Self::UdpSocket> {
        Ok(Box::pin(self.0.bind_udp_socket(addr)?))
    }

    fn new_delay(&self, dur: Duration) -> Self::Delay {
        Box::pin(self.0.new_delay(dur))
    }

    fn new_delay_at(&self, at: Instant) -> Self::Delay {
        Box::pin(self.0.new_delay_at(at))
    }

    fn new_interval(&self, dur: Duration) -> Self::Interval {
        Box::pin(self.0.new_interval(dur))
    }
}

impl<R: TcpListener> TcpListener for Dyn<R> {
    type TcpStream = BoxTcpStream;

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.0.local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Self::TcpStream>> {
        let stream = ready!(self.get_pin_mut().poll_accept(cx))?;
        Poll::Ready(Ok(Box::pin(stream) as _))
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.0.as_raw_fd()
    }
}
