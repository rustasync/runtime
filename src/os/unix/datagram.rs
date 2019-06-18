use futures::prelude::*;

use std::io;
use std::os::unix::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

/// A Unix Datagram socket.
#[derive(Debug)]
pub struct UnixDatagram {
    inner: Pin<Box<dyn runtime_raw::UnixDatagram>>,
}

impl UnixDatagram {
    /// Creates a Unix Datagram socket from the given path.
    pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_owned();
        let inner = runtime_raw::current_runtime().bind_unix_datagram(&path)?;
        Ok(UnixDatagram { inner })
    }

    /// Returns the local address that this listener is bound to.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    /// Sends data on the socket to the given address.
    pub fn send_to<'socket, 'buf, P: AsRef<Path>>(
        &'socket mut self,
        buf: &'buf [u8],
        path: P,
    ) -> SendToFuture<'socket, 'buf> {
        let path = path.as_ref().to_owned();
        SendToFuture {
            buf,
            path,
            socket: self,
        }
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read and the origin.
    pub fn recv_from<'socket, 'buf>(
        &'socket mut self,
        buf: &'buf mut [u8],
    ) -> RecvFromFuture<'socket, 'buf> {
        RecvFromFuture { buf, socket: self }
    }
}

/// The future returned by [`UnixDatagram::send_to`].
///
/// On success, returns the number of bytes written.
///
/// [`UnixDatagram::send_to`]: struct.UnixDatagram.html#method.send_to
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct SendToFuture<'socket, 'buf> {
    /// The open socket we use to send the message from.
    socket: &'socket mut UnixDatagram,
    /// The message we're trying to send.
    buf: &'buf [u8],
    /// The address we'll try to connect to.
    path: PathBuf,
}

impl<'socket, 'buf> Future for SendToFuture<'socket, 'buf> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let SendToFuture { socket, buf, path } = &mut *self;
        let poll = socket.inner.as_mut().poll_send_to(cx, buf, &path);
        poll
    }
}

/// The future returned by [`UnixDatagram::recv_from`].
///
/// On success, returns the number of bytes read and the origin.
///
/// [`UnixDatagram::recv_from`]: struct.UnixDatagram.html#method.recv_from
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct RecvFromFuture<'socket, 'buf> {
    socket: &'socket mut UnixDatagram,
    buf: &'buf mut [u8],
}

impl<'socket, 'buf> Future for RecvFromFuture<'socket, 'buf> {
    type Output = io::Result<(usize, SocketAddr)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let RecvFromFuture { socket, buf } = &mut *self;
        socket.inner.as_mut().poll_recv_from(cx, buf)
    }
}
