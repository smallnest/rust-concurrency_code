#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    use std::thread;

    #[test]
    fn test_atomic() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // 启动10个线程，每个线程尝试增加计数器1000次
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    // 错误：读取-修改-写入不是原子操作
                    let current = counter_clone.load(Ordering::Relaxed);
                    counter_clone.store(current + 1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Final counter: {}", counter.load(Ordering::Relaxed));
    }
}
