use std::io;
use std::net::Shutdown;
use std::os::unix::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};

use romio::raw::AsyncDatagram;

#[derive(Debug)]
pub(super) struct UnixDatagram {
    pub(super) romio_datagram: romio::uds::UnixDatagram,
}

impl runtime_raw::UnixDatagram for UnixDatagram {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_datagram.local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.romio_datagram.peer_addr()
    }

    fn poll_send_to(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &PathBuf,
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.romio_datagram).poll_send_to(cx, buf, receiver)
    }

    fn poll_recv_from(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        Pin::new(&mut self.romio_datagram).poll_recv_from(cx, buf)
    }

    fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.romio_datagram.shutdown(how)
    }
}
