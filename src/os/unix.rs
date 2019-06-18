//! Unix extensions

mod datagram;

/// Networking
pub mod net {
    pub use super::datagram::UnixDatagram;
}
