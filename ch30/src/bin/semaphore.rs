// 简化的信号量实现
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Semaphore {
    count: AtomicUsize,
}

impl Semaphore {
    pub fn new(count: usize) -> Self {
        Self { count: AtomicUsize::new(count) }
    }
    
    pub fn acquire(&self) {
        loop {
            let current = self.count.load(Ordering::Relaxed);
            if current == 0 {
                std::thread::yield_now();
                continue;
            }
            
            if self.count.compare_exchange(
                current, current - 1, Ordering::Acquire, Ordering::Relaxed
            ).is_ok() {
                break;
            }
        }
    }
    
    pub fn release(&self) {
        self.count.fetch_add(1, Ordering::Release);
    }
}

fn main() {
    use std::sync::Arc;
    
    let semaphore = Arc::new(Semaphore::new(3));
    
    // 模拟多个线程使用信号量
    for i in 0..10 {
        let semaphore_clone = semaphore.clone();
        std::thread::spawn(move || {
            semaphore_clone.acquire();
            println!("Thread {} acquired the semaphore", i);
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("Thread {} releasing the semaphore", i);
            semaphore_clone.release();
        });
    }
    
    // 等待所有线程完成
    std::thread::sleep(std::time::Duration::from_secs(10));
}