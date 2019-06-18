use std::io;
use std::net::Shutdown;
use std::os::unix::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::path::PathBuf;

#[derive(Debug)]
pub(super) struct UnixDatagram {
    pub(super) tokio_datagram: tokio::net::UnixDatagram,
}

impl runtime_raw::UnixDatagram for UnixDatagram {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_datagram.local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_datagram.peer_addr()
    }

    fn poll_send_to(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &PathBuf,
    ) -> Poll<io::Result<usize>> {
        let socket = unsafe { &mut self.get_unchecked_mut().tokio_datagram };
        match socket.poll_send_to(&buf, &receiver)? {
            futures01::Async::Ready(size) => Poll::Ready(Ok(size)),
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    fn poll_recv_from(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        let socket = unsafe { &mut self.get_unchecked_mut().tokio_datagram };
        match socket.poll_recv_from(buf)? {
            futures01::Async::Ready((size, addr)) => Poll::Ready(Ok((size, addr))),
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.tokio_datagram.shutdown(how)
    }
}
