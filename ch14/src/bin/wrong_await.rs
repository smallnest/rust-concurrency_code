async fn my_async_function() -> i32 { 42 }

#[tokio::main]
async fn main() {
    
    // 错误：忘记 .await，函数不会执行
    my_async_function();

    // 正确
    let result = my_async_function().await;
    println!("正确的异步操作的结果是：{}", result);
}