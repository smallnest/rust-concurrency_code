use std::thread;
use tokio;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("异步闭包的结果是：{}", thread::current().name().unwrap());
    });
}