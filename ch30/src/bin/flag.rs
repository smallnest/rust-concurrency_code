use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    // 使用 Arc<AtomicBool> 在线程间共享标志
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let handle = thread::spawn(move || {
        // 工作线程循环检查标志
        // 使用 Acquire 加载确保能观察到主线程的 Release 写入
        while running_clone.load(Ordering::Acquire) {
            println!("Working...");
            thread::sleep(Duration::from_millis(500));
        }
        println!("Worker thread stopping.");
    });

    // 主线程等待一段时间后设置标志
    thread::sleep(Duration::from_secs(2));
    println!("Main thread signaling stop.");
    // 使用 Release 存储确保此写入对工作线程的 Acquire 加载可见
    running.store(false, Ordering::Release);

    handle.join().unwrap();
    println!("Main thread finished.");
}
