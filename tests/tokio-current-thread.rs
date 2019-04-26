#![feature(async_await, await_macro)]

#[runtime::test(runtime_tokio::TokioCurrentThread)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Tokio current-thread");
        42
    });
    assert_eq!(await!(handle), 42);
}
