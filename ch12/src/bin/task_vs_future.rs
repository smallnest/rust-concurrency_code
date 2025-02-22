use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task;

// 使用 std::future 实现一个简单的异步操作
struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 模拟异步操作，这里直接返回一个完成状态
        Poll::Ready(42)
    }
}

#[tokio::main]
async fn main() {
    // 使用 std::future
    let my_future = MyFuture;
    let result = my_future.await;
    println!("std::future result: {}", result);

    // 使用 Tokio::task
    let task = task::spawn(async {
        // 在 Tokio 运行时中执行异步操作
        42
    });
    let result = task.await.unwrap();
    println!("Tokio::task result: {}", result);
}