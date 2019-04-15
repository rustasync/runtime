#![feature(async_await, await_macro, futures_api)]

use runtime_native::Native;

#[runtime::test(Native)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Native");
        42
    });
    assert_eq!(await!(handle), 42);
}
