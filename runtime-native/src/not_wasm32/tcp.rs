use futures::prelude::*;
use romio::raw::AsyncReady;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(crate) struct TcpStream {
    pub romio_stream: romio::tcp::TcpStream,
}

#[derive(Debug)]
pub(crate) struct TcpListener {
    pub romio_listener: romio::tcp::TcpListener,
}

impl runtime_raw::TcpStream for TcpStream {
    fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_stream.local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.romio_stream.peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.romio_stream.shutdown(how)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.romio_stream.as_raw_fd()
    }
}

impl AsyncRead for TcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.romio_stream).poll_read(cx, &mut buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.romio_stream).poll_write(cx, &buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.romio_stream).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.romio_stream).poll_close(cx)
    }
}

impl runtime_raw::TcpListener for TcpListener {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_listener.local_addr()
    }

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        Pin::new(&mut self.romio_listener)
            .poll_ready(cx)
            .map_ok(|(romio_stream, _)| {
                Box::pin(TcpStream { romio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
            })
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.romio_listener.as_raw_fd()
    }
}
