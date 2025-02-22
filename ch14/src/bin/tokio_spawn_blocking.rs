use tokio::task;

#[tokio::main]
async fn main() {
    // 执行一些异步 I/O 操作
    println!("开始 I/O 操作");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("I/O 操作完成");

    // 执行 CPU 密集型任务
    let result = task::spawn_blocking(move || {
        println!("开始 CPU 密集型任务");
        // 执行一些 CPU 密集型计算
        let mut sum = 0u64;
        for i in 0..100000000 {
            sum += i;
        }
        println!("CPU 密集型任务完成");
        sum
    }).await.unwrap();

    println!("计算结果：{}", result);
}