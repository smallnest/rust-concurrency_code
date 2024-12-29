async fn my_async_function() -> i32 {
    10
}

#[tokio::main]
async fn main() {
    let result = my_async_function().await;
    println!("Result: {}", result);
}