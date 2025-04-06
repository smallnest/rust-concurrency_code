use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 创建一个信号量，初始许可数为3。
    // 并且使用 `Arc` 来共享信号量的所有权。
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    // 启动5个异步任务，每个任务都请求一个许可。
    // 由于信号量的初始许可数为3，所以只有3个任务可以同时获得许可。
    // 其他任务会被阻塞，直到有许可可用。
    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // 在这个任务中拥有许可
            println!("任务拥有许可，正在执行...");
            // 执行一些业务逻辑

            // 当任务完成时，释放许可
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}