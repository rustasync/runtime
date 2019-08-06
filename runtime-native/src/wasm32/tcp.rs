use futures::io::{AsyncRead, AsyncWrite};

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use super::Unimplemented;

impl runtime_raw::TcpStream for Unimplemented {
    fn poll_write_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unimplemented!()
    }

    fn poll_read_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unimplemented!()
    }

    fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!()
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!()
    }

    fn shutdown(&self, _how: std::net::Shutdown) -> std::io::Result<()> {
        unimplemented!()
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        unimplemented!()
    }
}

impl AsyncRead for Unimplemented {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        unimplemented!()
    }
}

impl AsyncWrite for Unimplemented {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        unimplemented!()
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unimplemented!()
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        unimplemented!()
    }
}

impl runtime_raw::TcpListener for Unimplemented {
    type TcpStream = Unimplemented;

    fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<io::Result<Self::TcpStream>> {
        unimplemented!()
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        unimplemented!()
    }
}
