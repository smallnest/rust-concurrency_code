use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 定义一个信号量，初始许可数为10,也是我们的发送并发量。
    // 使用 `Arc` 来共享信号量的所有权。
    let semaphore = Arc::new(Semaphore::new(10));
    // 创建非常多的异步任务，每个任务都尝试获取信号量的许可。
    let mut jhs = Vec::new();
    for task_id in 0..100 {
        let semaphore = semaphore.clone();
        let jh = tokio::spawn(async move {
            // 发送请求之前，先获取信号量的许可。
            let _permit = semaphore.acquire().await.unwrap();
            // 发送请求
            let response = send_request(task_id).await;
            // 请求完成后，释放许可。
            drop(_permit);
            
            // 返回请求的响应
            response
        });
        jhs.push(jh);
    }
    
    // 等待所有任务完成
    let mut responses = Vec::new();
    for jh in jhs {
        let response = jh.await.unwrap();
        responses.push(response);
    }
}

async fn send_request(task_id: usize) -> String {
    // 模拟发送请求的延迟
    // 实际的请求发送逻辑会在这里实现
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    format!("Response from task {}", task_id)
}