#![feature(test, async_await)]
#![warn(rust_2018_idioms)]

extern crate test;

#[macro_use]
mod common;

mod native {
    benchmark_suite!(runtime_native::Native);
}
