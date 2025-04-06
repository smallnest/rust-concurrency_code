use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                // fetch_add 进行原子递增
                // Relaxed 通常足够用于简单计数，因为我们只关心最终总和，
                // 不关心增量操作之间的确切顺序或它们与其他变量的同步。
                // 如果计数器用于控制对其他数据的访问，可能需要更强的顺序。
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            println!("Thread {} finished counting.", i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 读取最终结果时，通常需要 Acquire 或 SeqCst 来确保看到所有线程的最终写入。
    let final_count = counter.load(Ordering::Acquire);
    println!("Final count: {}", final_count); // 应为 500
}
