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
use futures::{future::FutureObj, task::SpawnError};
use lazy_static::lazy_static;

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

mod tcp;
mod udp;

use tcp::{TcpListener, TcpStream};
use udp::UdpSocket;

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
    fn spawn_obj(&self, fut: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        JULIEX_THREADPOOL.spawn(fut);
        Ok(())
    }

    fn connect_tcp_stream(
        &self,
        addr: &SocketAddr,
    ) -> Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn runtime_raw::TcpStream>>>> + Send>>
    {
        let romio_connect = romio::TcpStream::connect(addr);
        let connect = romio_connect.map(|res| {
            res.map(|romio_stream| {
                Box::pin(TcpStream { romio_stream }) as Pin<Box<dyn runtime_raw::TcpStream>>
            })
        });
        Box::pin(connect)
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
