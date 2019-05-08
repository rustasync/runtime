//! A cross-platform asynchronous [Runtime](https://github.com/rustasync/runtime). See the [Runtime
//! documentation](https://docs.rs/runtime) for more details.

#![feature(async_await)]
#![deny(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms
)]

#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
mod wasm32;
#[cfg(all(feature = "wasm-bindgen", target_arch = "wasm32"))]
pub use wasm32::Native;

#[cfg(not(target_arch = "wasm32"))]
mod not_wasm32;
#[cfg(not(target_arch = "wasm32"))]
pub use not_wasm32::Native;
