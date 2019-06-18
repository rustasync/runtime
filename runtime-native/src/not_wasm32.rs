use futures::prelude::*;
use futures::{future::BoxFuture, task::SpawnError};
use futures_timer::{Delay as AsyncDelay, Interval as AsyncInterval};
use lazy_static::lazy_static;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};
use std::path::Path;

mod tcp;
mod time;
mod udp;
mod unix;

use tcp::{TcpListener, TcpStream};
use time::{Delay, Interval};
use udp::UdpSocket;
use unix::UnixDatagram;

lazy_static! {
    static ref JULIEX_THREADPOOL: juliex::ThreadPool = {
        juliex::ThreadPool::with_setup(|| {
            runtime_raw::set_runtime(&Native);
        })
    };
}

/// The Native runtime.
#[derive(Debug)]
pub struct Native;

impl runtime_raw::Runtime for Native {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        JULIEX_THREADPOOL.spawn_boxed(fut.into());
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> BoxFuture<'static, io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        let romio_connect = romio::TcpStream::connect(addr);
        let connect = romio_connect.map(|res| {
            res.map(|romio_stream| {
                Box::pin(TcpStream { romio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
            })
        });
        connect.boxed()
    }

    fn bind_tcp_listener(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::TcpListener>>> {
        let romio_listener = romio::TcpListener::bind(&addr)?;
        Ok(Box::pin(TcpListener { romio_listener }))
    }

    fn bind_udp_socket(
        &self,
        addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::UdpSocket>>> {
        let romio_socket = romio::UdpSocket::bind(&addr)?;
        Ok(Box::pin(UdpSocket { romio_socket }))
    }

    fn bind_unix_datagram(&self, addr: &Path) -> io::Result<Pin<Box<dyn runtime_raw::UnixDatagram>>> {
        let romio_datagram = romio::uds::UnixDatagram::bind(&addr)?;
        Ok(Box::pin(UnixDatagram { romio_datagram }))
    }

    fn new_delay(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Delay>> {
        let async_delay = AsyncDelay::new(dur);
        Box::pin(Delay { async_delay })
    }

    fn new_delay_at(&self, at: Instant) -> Pin<Box<dyn runtime_raw::Delay>> {
        let async_delay = AsyncDelay::new_at(at);
        Box::pin(Delay { async_delay })
    }

    fn new_interval(&self, dur: Duration) -> Pin<Box<dyn runtime_raw::Interval>> {
        let async_interval = AsyncInterval::new(dur);
        Box::pin(Interval { async_interval })
    }
}
