use futures::prelude::*;
use futures::{future::BoxFuture, task::SpawnError};
// use futures::compat::*;

use std::io;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

mod tcp;
mod time;
mod udp;

/// The Native runtime.
#[derive(Debug)]
pub struct Native;

#[derive(Debug)]
struct Unimplemented;

impl Unimplemented {
    pub fn new(msg: &'static str) -> Self {
        panic!(msg)
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
        let fut = fut.unit_error().compat();
        wasm_bindgen_futures::spawn_local(fut);
        Ok(())
    }

    fn connect_tcp_stream(&self, _addr: &SocketAddr) -> Self::ConnectTcpStream {
        async {
            Ok(Unimplemented::new(
                "Connecting TCP streams is currently not supported in wasm",
            ))
        }
    }

    fn bind_tcp_listener(&self, _addr: &SocketAddr) -> io::Result<Self::TcpListener> {
        Ok(Unimplemented::new(
            "Binding TCP listeners is currently not supported in wasm",
        ))
    }

    fn bind_udp_socket(&self, _addr: &SocketAddr) -> io::Result<Self::UdpSocket> {
        Ok(Unimplemented::new(
            "Binding UDP sockets is currently not supported in wasm",
        ))
    }

    fn new_delay(&self, _dur: Duration) -> Self::Delay {
        Unimplemented::new("Timers are currently not supported in wasm")
    }

    fn new_delay_at(&self, _at: Instant) -> Self::Delay {
        Unimplemented::new("Timers are currently not supported in wasm")
    }

    fn new_interval(&self, _dur: Duration) -> Self::Interval {
        Unimplemented::new("Timers are currently not supported in wasm")
    }
}
