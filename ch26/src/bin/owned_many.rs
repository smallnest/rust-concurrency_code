use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 创建一个信号量，初始许可数为10。
    // 并且使用 `Arc` 来共享信号量的所有权。
    let semaphore = Arc::new(Semaphore::new(10));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        // 请求2个许可
        let permit = semaphore.clone().acquire_many_owned(2).await.unwrap();
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