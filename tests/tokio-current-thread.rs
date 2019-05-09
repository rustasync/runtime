#![feature(async_await)]

#[runtime::test(runtime_tokio::TokioCurrentThread)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Tokio current-thread");
        42
    });
    assert_eq!(handle.await, 42);
}
