use  tokio::join;

async fn do_stuff_async() -> i32 {
    println!("do stuff");
    1
}

async fn more_async_work() -> &'static str {
    println!("more work");
    "hello join"
}

#[tokio::main]
async fn main() {
    let (first, second) = join!(
        do_stuff_async(),
        more_async_work());

    println!("first: {}, second: {}", first, second);
}