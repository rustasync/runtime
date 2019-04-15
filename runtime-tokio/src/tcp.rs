use futures::prelude::*;

use futures::compat::Compat01As03;
use futures01;

use std::io;
use std::net::SocketAddr;
use std::task::{Poll, Waker};

#[derive(Debug)]
pub(crate) struct TcpStream {
    pub tokio_stream: tokio::net::tcp::TcpStream,
}

#[derive(Debug)]
pub(crate) struct TcpListener {
    pub tokio_listener: tokio::net::tcp::TcpListener,
}

impl runtime_raw::TcpStream for TcpStream {
    fn poll_write_ready(&self, _waker: &Waker) -> Poll<io::Result<()>> {
        match self.tokio_stream.poll_write_ready() {
            Err(e) => Poll::Ready(Err(e)),
            Ok(futures01::Async::Ready(_)) => Poll::Ready(Ok(())),
            Ok(futures01::Async::NotReady) => Poll::Pending,
        }
    }

    fn poll_read_ready(&self, _waker: &Waker) -> Poll<io::Result<()>> {
        let mask = mio::Ready::readable();
        match self.tokio_stream.poll_read_ready(mask) {
            Err(e) => Poll::Ready(Err(e)),
            Ok(futures01::Async::Ready(_)) => Poll::Ready(Ok(())),
            Ok(futures01::Async::NotReady) => Poll::Pending,
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
    fn poll_read(&mut self, waker: &Waker, mut buf: &mut [u8]) -> Poll<io::Result<usize>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        stream.poll_read(&waker, &mut buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(&mut self, waker: &Waker, buf: &[u8]) -> Poll<io::Result<usize>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        stream.poll_write(&waker, &buf)
    }

    fn poll_flush(&mut self, waker: &Waker) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        stream.poll_flush(&waker)
    }

    fn poll_close(&mut self, waker: &Waker) -> Poll<io::Result<()>> {
        let mut stream = Compat01As03::new(&self.tokio_stream);
        stream.poll_close(&waker)
    }
}

impl runtime_raw::TcpListener for TcpListener {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.tokio_listener.local_addr()
    }

    fn poll_accept(&mut self, _waker: &Waker) -> Poll<io::Result<Box<dyn runtime_raw::TcpStream>>> {
        match self.tokio_listener.poll_accept() {
            Err(e) => Poll::Ready(Err(e)),
            Ok(futures01::Async::Ready((tokio_stream, _))) => {
                let stream = Box::new(TcpStream { tokio_stream });
                Poll::Ready(Ok(stream))
            }
            Ok(futures01::Async::NotReady) => Poll::Pending,
        }
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.tokio_listener.as_raw_fd()
    }
}
