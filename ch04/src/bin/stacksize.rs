use std::thread;

fn main() {
    let handle = thread::Builder::new()
        .name("worker_thread".to_string())
        .stack_size(1024) // 设置栈大小为1M
        .spawn(|| {
            println!("这是一个工作线程");
            let a = vec![1; 1024 * 1024 * 1024]; // 创建一个非常大的向量，导致栈溢出
            println!("向量的大小: {}", a.len());
        })
        .unwrap();

    handle.join().unwrap();
}


