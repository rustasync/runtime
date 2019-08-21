use runtime_tokio::Tokio;

#[runtime::test(Tokio)]
async fn spawn() {
    let handle = runtime::spawn(async {
        println!("hello planet from Tokio");
        42
    });
    assert_eq!(handle.await, 42);
}
