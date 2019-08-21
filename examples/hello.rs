async fn say_hi() {
    println!("Hello world! 🤖");
}

#[runtime::main]
async fn main() {
    say_hi().await;
}
