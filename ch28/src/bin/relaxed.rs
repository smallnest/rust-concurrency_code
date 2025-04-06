use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // 创建一个可以在线程间共享的原子计数器
    let counter = Arc::new(AtomicUsize::new(0));
    
    // 创建多个线程，每个线程都增加计数器
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // 使用 Relaxed 内存序增加计数器100次
            for _ in 0..100 {
                // fetch_add 返回之前的值，并添加第一个参数到原子变量
                // Relaxed 表示我们只关心这个操作的原子性，不关心与其他内存操作的顺序
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 读取最终值，同样使用 Relaxed 内存序
    println!("最终计数: {}", counter.load(Ordering::Relaxed));
}