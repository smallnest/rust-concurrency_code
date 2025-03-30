use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let thread_count = 3;
    let barrier = Arc::new(Barrier::new(thread_count));

    let mut handles = vec![];

    for i in 0..thread_count {
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            for round in 0..2 { // 每个线程执行两轮
                println!("线程 {}，第 {} 轮，等待...", i, round);

                // 模拟线程执行一些任务
                thread::sleep(Duration::from_millis((i as u64 + 1) * 500));

                barrier_clone.wait(); // 等待所有线程到达屏障

                println!("线程 {}，第 {} 轮，继续执行...", i, round);

                // 模拟线程执行一些任务
                thread::sleep(Duration::from_millis((i as u64 + 1) * 500));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成！");
}