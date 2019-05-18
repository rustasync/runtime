use futures::prelude::*;
use futures::{future::BoxFuture, task::SpawnError};
// use futures::compat::*;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::Duration;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

/// The Native runtime.
#[derive(Debug)]
pub struct Native;

impl runtime_raw::Runtime for Native {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        use futures01::future::Future;
        let fut = fut.unit_error().compat();
        wasm_bindgen_futures::spawn_local(fut);
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        _addr: &SocketAddr,
    ) -> BoxFuture<'static, io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> {
        panic!("Connecting TCP streams is currently not supported in wasm");
    }

    fn bind_tcp_listener(
        &self,
        _addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::TcpListener>>> {
        panic!("Binding TCP listeners is currently not supported in wasm");
    }

    fn bind_udp_socket(
        &self,
        _addr: &SocketAddr,
    ) -> io::Result<Pin<Box<dyn runtime_raw::UdpSocket>>> {
        panic!("Binding UDP sockets is currently not supported in wasm");
    }

    fn new_delay(&self, _dur: Duration) -> Pin<Box<dyn runtime_raw::Delay>> {
        unimplemented!();
    }

    fn new_delay_at(&self, _dur: Duration) -> Pin<Box<dyn runtime_raw::Delay>> {
        unimplemented!();
    }

    fn new_interval(&self, _dur: Duration) -> Pin<Box<dyn runtime_raw::Interval>> {
        unimplemented!();
    }
}
