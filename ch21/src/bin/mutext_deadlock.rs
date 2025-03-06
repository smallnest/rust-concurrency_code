use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建两个互斥锁
    let lock1 = Arc::new(Mutex::new(1));
    let lock2 = Arc::new(Mutex::new(2));

    // 线程 A
    let a_lock1 = Arc::clone(&lock1);
    let a_lock2 = Arc::clone(&lock2);
    let thread_a = thread::spawn(move || {
        let lock1_guard = a_lock1.lock().unwrap();
        println!("Thread A acquired lock1");
        // 模拟一些工作
        thread::sleep(Duration::from_millis(100));
        let lock2_guard = a_lock2.lock().unwrap();
        println!("Thread A acquired lock2");
        // 线程A进行一些操作
        println!("Thread A do something");
    });

    // 线程 B
    let b_lock1 = Arc::clone(&lock1);
    let b_lock2 = Arc::clone(&lock2);
    let thread_b = thread::spawn(move || {
        let lock2_guard = b_lock2.lock().unwrap();
        println!("Thread B acquired lock2");
        // 模拟一些工作
        thread::sleep(Duration::from_millis(100));
        let lock1_guard = b_lock1.lock().unwrap();
        println!("Thread B acquired lock1");
        // 线程B进行一些操作
        println!("Thread B do something");
    });

    // 等待线程完成
    thread_a.join().unwrap();
    thread_b.join().unwrap();
}