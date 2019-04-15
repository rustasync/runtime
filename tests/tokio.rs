#![feature(async_await, await_macro, futures_api)]

use runtime_tokio::Tokio;

#[runtime::test(Tokio)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Tokio");
        42
    });
    assert_eq!(await!(handle), 42);
}
