use futures::prelude::*;
use pin_utils::unsafe_pinned;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(crate) struct TcpStream {
    pub tokio_stream: tokio::net::tcp::TcpStream,
}

impl TcpStream {
    unsafe_pinned!(tokio_stream: tokio::net::tcp::TcpStream);
}

#[derive(Debug)]
pub(crate) struct TcpListener {
    pub tokio_listener: tokio::net::tcp::TcpListener,
}

impl runtime_raw::TcpStream for TcpStream {
    fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_stream.local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_stream.peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.tokio_stream.shutdown(how)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.tokio_stream.as_raw_fd()
    }
}

impl AsyncRead for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        tokio::io::AsyncRead::poll_read(self.tokio_stream(), cx, &mut buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        tokio::io::AsyncWrite::poll_write(self.tokio_stream(), cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(self.tokio_stream(), cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(self.tokio_stream(), cx)
    }
}

impl runtime_raw::TcpListener for TcpListener {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_listener.local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        let listener = unsafe { &mut self.get_unchecked_mut().tokio_listener };
        Pin::new(&mut listener.accept().boxed())
            .poll(cx)
            .map_ok(|(tokio_stream, _)| {
                Box::pin(TcpStream { tokio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
            })
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.tokio_listener.as_raw_fd()
    }
}
