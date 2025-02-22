use std::thread;
use std::sync::mpsc;
use tokio::runtime::Runtime;
use ch14;

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
            let (nonce,_) = ch14::mine_zeros(2);
            println!("CPU 密集型任务完成");
            tx_clone.send(nonce).unwrap();
        });

        // 从通道接收结果
        let nonce = rx.recv().unwrap();
        println!("找到的nonce：{}", nonce);
    });
}