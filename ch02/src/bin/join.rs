// 多个子线程join
use std::thread;

fn main() {
    // 创建10个线程
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                println!("线程#{}", i);
            })
        })
        .collect();

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }
}
