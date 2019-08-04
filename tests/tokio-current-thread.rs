#![feature(async_await)]

#[runtime::test(runtime_tokio::TokioCurrentThread)]
async fn spawn() {
    let handle = runtime::task::spawn_remote(async {
        println!("hello planet from Tokio current-thread");
        42
    });
    assert_eq!(handle.await, 42);
}
