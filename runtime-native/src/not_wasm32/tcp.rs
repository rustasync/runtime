use futures::prelude::*;
use romio::raw::{AsyncReadReady, AsyncReady, AsyncWriteReady};

use super::Compat;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

impl runtime_raw::TcpStream for Compat<romio::TcpStream> {
    fn poll_write_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_write_ready(cx).map_ok(|_| ())
    }

    fn poll_read_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_read_ready(cx).map_ok(|_| ())
    }

    fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.get_ref().shutdown(how)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.get_ref().as_raw_fd()
    }
}

impl AsyncRead for Compat<romio::TcpStream> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_read(cx, &mut buf)
    }
}

impl AsyncWrite for Compat<romio::TcpStream> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.get_pin_mut().poll_write(cx, &buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_pin_mut().poll_close(cx)
    }
}

impl runtime_raw::TcpListener for Compat<romio::TcpListener> {
    type TcpStream = Compat<romio::TcpStream>;

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Self::TcpStream>> {
        self.get_pin_mut()
            .poll_ready(cx)
            .map_ok(|(romio_stream, _)| Compat::new(romio_stream))
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.get_ref().as_raw_fd()
    }
}
