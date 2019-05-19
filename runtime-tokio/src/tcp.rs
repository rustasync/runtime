use futures::prelude::*;

use futures::compat::Compat01As03;
use futures01;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(crate) struct TcpStream {
    pub tokio_stream: tokio::net::tcp::TcpStream,
}

#[derive(Debug)]
pub(crate) struct TcpListener {
    pub tokio_listener: tokio::net::tcp::TcpListener,
}

impl runtime_raw::TcpStream for TcpStream {
    fn poll_write_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.tokio_stream.poll_write_ready()? {
            futures01::Async::Ready(_) => Poll::Ready(Ok(())),
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    fn poll_read_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mask = mio::Ready::readable();
        match self.tokio_stream.poll_read_ready(mask)? {
            futures01::Async::Ready(_) => Poll::Ready(Ok(())),
            futures01::Async::NotReady => Poll::Pending,
        }
    }

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
        let mut stream = Compat01As03::new(&self.tokio_stream);
        Pin::new(&mut stream).poll_read(cx, &mut buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        Pin::new(&mut stream).poll_write(cx, &buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        Pin::new(&mut stream).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        Pin::new(&mut stream).poll_close(cx)
    }
}

impl runtime_raw::TcpListener for TcpListener {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_listener.local_addr()
    }

    fn poll_accept(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        let listener = unsafe { &mut self.get_unchecked_mut().tokio_listener };
        match listener.poll_accept()? {
            futures01::Async::Ready((tokio_stream, _)) => {
                let stream = Box::pin(TcpStream { tokio_stream });
                Poll::Ready(Ok(stream))
            }
            futures01::Async::NotReady => Poll::Pending,
        }
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.tokio_listener.as_raw_fd()
    }
}
