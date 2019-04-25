#![feature(async_await, await_macro)]

async fn say_hi() {
    println!("Hello world! 🤖");
}

#[runtime::main]
async fn main() {
    await!(say_hi());
}
