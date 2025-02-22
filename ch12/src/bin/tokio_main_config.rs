use std::thread;
use tokio;

// #[tokio::main]

// // 单线程
// #[tokio::main(flavor = "current_thread")]

// // 多线程,指定工作线程数
// #[tokio::main(worker_threads = 4)]

#[tokio::main(
    worker_threads = 4,
    flavor = "multi_thread"
)]
async fn main() {
    
    println!("异步闭包的结果是：{}", thread::current().name().unwrap());
}