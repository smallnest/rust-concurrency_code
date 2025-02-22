async fn my_async_function(x: i32) -> i32 {
    // 一些异步操作，例如网络请求或文件 I/O
    println!("开始异步操作，输入值为：{}", x);
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await; // 模拟耗时操作
    println!("异步操作完成");
    x * 2
}

#[tokio::main] // 使用 tokio 运行时
async fn main() {
    let future = my_async_function(5); // 创建一个 Future，但不会立即执行
    println!("main 函数中，等待 future 完成...");
    let result = future.await; // 等待 Future 完成并获取结果
    println!("异步操作的结果是：{}", result);
}