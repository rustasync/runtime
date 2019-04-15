//! Networking primitives for asynchronous TCP/UDP communication.
//!
//! This module provides networking functionality for the Transmission Control and User
//! Datagram Protocols, as well as types for IP and socket addresses.
//!
//! # Organization
//!
//! * [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
//! * [`UdpSocket`] provides functionality for communication over UDP
//! * Other types are return or parameter types for various methods in this module
//!
//! [`TcpListener`]: struct.TcpListener.html
//! [`TcpStream`]: struct.TcpStream.html
//! [`UdpSocket`]: struct.UdpSocket.html

pub mod tcp;
pub mod udp;

#[doc(inline)]
pub use tcp::{TcpListener, TcpStream};

#[doc(inline)]
pub use udp::UdpSocket;
