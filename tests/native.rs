#![feature(async_await)]

use runtime_native::Native;

#[runtime::test(Native)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Native");
        42
    });
    assert_eq!(handle.await, 42);
}
