use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个原子计数器作为信号量（表示可用资源数量）
    let semaphore = Arc::new(AtomicUsize::new(3)); // 初始有3个资源可用
    let mut handles = vec![];
    
    // 创建5个工作线程，它们都需要获取资源
    for id in 1..=5 {
        let sem = Arc::clone(&semaphore);
        
        let handle = thread::spawn(move || {
            println!("线程 {} 尝试获取资源", id);
            
            // 尝试获取资源（减少信号量）
            let mut acquired = false;
            while !acquired {
                // 当前值
                let current = sem.load(Ordering::Relaxed);
                if current > 0 {
                    // compare_exchange_weak尝试原子地将值从current更新为current-1
                    // 使用AcqRel: 成功时同时具有Acquire和Release语义
                    match sem.compare_exchange_weak(
                        current, 
                        current - 1,
                        Ordering::AcqRel,  // 成功时使用AcqRel
                        Ordering::Acquire  // 失败时使用Acquire
                    ) {
                        Ok(_) => {
                            acquired = true;
                            println!("线程 {} 获得资源，剩余资源: {}", id, current - 1);
                        }
                        Err(_) => {
                            // 竞争失败，其他线程修改了值，重试
                            thread::yield_now();
                        }
                    }
                } else {
                    // 没有可用资源，稍后重试
                    println!("线程 {} 等待资源...", id);
                    thread::sleep(Duration::from_millis(10));
                }
            }
            
            // 模拟使用资源
            println!("线程 {} 正在使用资源...", id);
            thread::sleep(Duration::from_millis(50));
            
            // 释放资源（增加信号量）
            sem.fetch_add(1, Ordering::Release);
            println!("线程 {} 释放资源", id);
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}