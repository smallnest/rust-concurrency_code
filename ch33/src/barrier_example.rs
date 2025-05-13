#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(4)); // 期望4个线程
        let mut handles = vec![];

        // 错误：只启动3个线程
        for i in 0..3 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                println!("Thread {}: Waiting", i);
                barrier_clone.wait();
                println!("Thread {}: Passed barrier", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
