#![feature(test, async_await)]
#![warn(rust_2018_idioms)]

extern crate test;

#[macro_use]
mod common;

mod tokio {
    benchmark_suite!(runtime_tokio::Tokio);
}

mod tokio_current_thread {
    benchmark_suite!(runtime_tokio::TokioCurrentThread);
}
