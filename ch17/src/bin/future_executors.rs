use futures::executor::block_on;
use futures::future::ready;

async fn my_async_function() -> i32 {
    let future1 = ready(10); // 创建一个立即完成的 Future
    let future2 = async { 20 }; // 创建一个异步 Future

    let result1 = future1.await; // 等待 future1 完成
    let result2 = future2.await; // 等待 future2 完成

    result1 + result2
}

fn main() {
    let result = block_on(my_async_function()); // 执行异步函数
    println!("Result: {}", result); // 输出：Result: 30
}