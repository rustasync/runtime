//! A cross-platform asynchronous [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(async_await, await_macro)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

use futures::prelude::*;
use futures::{future::BoxFuture, task::SpawnError};
#[cfg(not(target_arch = "wasm32"))]
use lazy_static::lazy_static;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;

use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

#[cfg(not(target_arch = "wasm32"))]
mod tcp;
#[cfg(not(target_arch = "wasm32"))]
mod udp;

#[cfg(not(target_arch = "wasm32"))]
use tcp::{TcpListener, TcpStream};
#[cfg(not(target_arch = "wasm32"))]
use udp::UdpSocket;

#[cfg(not(target_arch = "wasm32"))]
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

// Unix + Windows
#[cfg(not(target_arch = "wasm32"))]
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
}

#[cfg(target_arch = "wasm32")]
impl runtime_raw::Runtime for Native {
    fn spawn_boxed(&self, fut: BoxFuture<'static, ()>) -> Result<(), SpawnError> {
        use futures01::future::Future;
        let fut = fut.unit_error().compat()
            .map(|_| JsValue::undefined())
            .map_err(|_| JsValue::undefined());
        future_to_promise(fut);
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
}
