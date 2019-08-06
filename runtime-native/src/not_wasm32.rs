use futures::prelude::*;
use futures::{future::BoxFuture, task::SpawnError};
use futures_timer::{Delay as AsyncDelay, Interval as AsyncInterval};
use lazy_static::lazy_static;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

lazy_static! {
    static ref JULIEX_THREADPOOL: juliex::ThreadPool = {
        juliex::ThreadPool::with_setup(|| {
            runtime_raw::set_runtime(Native);
        })
    };
}

/// The Native runtime.
#[derive(Debug)]
pub struct Native;

#[derive(Debug)]
struct Compat<T>(T);

impl<T> Compat<T> {
    fn new(inner: T) -> Self {
        Self(inner)
    }

    fn get_ref(&self) -> &T {
        &self.0
    }

    #[allow(unsafe_code)]
    fn get_pin_mut<'a>(self: Pin<&'a mut Self>) -> Pin<&'a mut T> {
        unsafe { Pin::new_unchecked(&mut Pin::get_unchecked_mut(self).0) }
    }
}

impl runtime_raw::Runtime for Native {
    type TcpStream = impl runtime_raw::TcpStream;
    type TcpListener = impl runtime_raw::TcpListener<TcpStream = Self::TcpStream>;
    type UdpSocket = impl runtime_raw::UdpSocket;
    type Delay = impl runtime_raw::Delay;
    type Interval = impl runtime_raw::Interval;

    type ConnectTcpStream = impl Future<Output = io::Result<Self::TcpStream>> + Send;

    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        JULIEX_THREADPOOL.spawn_boxed(fut);
        Ok(())
    }

    fn connect_tcp_stream(&self, addr: &SocketAddr) -> Self::ConnectTcpStream {
        romio::TcpStream::connect(addr).map_ok(Compat::new)
    }

    fn bind_tcp_listener(&self, addr: &SocketAddr) -> io::Result<Self::TcpListener> {
        romio::TcpListener::bind(&addr).map(Compat::new)
    }

    fn bind_udp_socket(&self, addr: &SocketAddr) -> io::Result<Self::UdpSocket> {
        romio::UdpSocket::bind(&addr).map(Compat::new)
    }

    fn new_delay(&self, dur: Duration) -> Self::Delay {
        Compat::new(AsyncDelay::new(dur))
    }

    fn new_delay_at(&self, at: Instant) -> Self::Delay {
        Compat::new(AsyncDelay::new_at(at))
    }

    fn new_interval(&self, dur: Duration) -> Self::Interval {
        Compat::new(AsyncInterval::new(dur))
    }
}
