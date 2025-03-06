use tokio::task;
use tokio::time::Duration;

async fn handle_request() {
    println!("开始处理请求");
    // tokio::time::sleep(Duration::from_secs(1)).await; // 正确：使用 tokio::time::sleep
    std::thread::sleep(Duration::from_secs(1)); // 错误：使用 std::thread::sleep
    println!("请求处理完成");
}

#[tokio::main(flavor = "current_thread")] // 使用 tokio::main 宏，单线程模式
async fn main() {
    let start = std::time::Instant::now();

    // 启动多个并发任务
    let handles = (0..10).map(|_| {
        task::spawn(handle_request())
    }).collect::<Vec<_>>();

    // 等待所有任务完成（可选）
    for handle in handles {
        handle.await.unwrap();
    }

    println!("所有请求处理完成，耗时 {:?}", start.elapsed());
}