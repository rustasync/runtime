use futures::prelude::*;

use futures::compat::Compat01As03;
use futures01;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Compat;

impl runtime_raw::TcpStream for Compat<tokio::net::TcpStream> {
    fn poll_write_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_ref().poll_write_ready()? {
            futures01::Async::Ready(_) => Poll::Ready(Ok(())),
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    fn poll_read_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mask = mio::Ready::readable();
        match self.get_ref().poll_read_ready(mask)? {
            futures01::Async::Ready(_) => Poll::Ready(Ok(())),
            futures01::Async::NotReady => Poll::Pending,
        }
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

impl AsyncRead for Compat<tokio::net::TcpStream> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut stream = Compat01As03::new(self.get_ref());
        Pin::new(&mut stream).poll_read(cx, &mut buf)
    }
}

impl AsyncWrite for Compat<tokio::net::TcpStream> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut stream = Compat01As03::new(self.get_ref());
        Pin::new(&mut stream).poll_write(cx, &buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(self.get_ref());
        Pin::new(&mut stream).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(self.get_ref());
        Pin::new(&mut stream).poll_close(cx)
    }
}

impl runtime_raw::TcpListener for Compat<tokio::net::TcpListener> {
    type TcpStream = Compat<tokio::net::TcpStream>;

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.get_ref().local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<io::Result<Self::TcpStream>> {
        match self.get_mut().get_mut().poll_accept()? {
            futures01::Async::Ready((tokio_stream, _)) => {
                Poll::Ready(Ok(Compat::new(tokio_stream)))
            }
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.get_ref().as_raw_fd()
    }
}
