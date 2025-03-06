use std::thread;
use std::sync::mpsc;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let (tx, rx) = mpsc::channel();

    rt.block_on(async {
        // 执行一些异步 I/O 操作
        println!("开始 I/O 操作");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("I/O 操作完成");

        // 将 CPU 密集型任务发送到线程池
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("开始 CPU 密集型任务");
            // 执行一些 CPU 密集型计算
            let mut sum = 0u64;
            for i in 0..100000000 {
                sum += i;
            }
            println!("CPU 密集型任务完成");
            tx_clone.send(sum).unwrap();
        });

        // 从通道接收结果
        let result = rx.recv().unwrap();
        println!("计算结果：{}", result);
    });
}