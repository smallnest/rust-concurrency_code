use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let rwlock = Arc::new(RwLock::new(0));

    // 持续的读线程
    for _ in 0..10 {
        let read_lock = rwlock.clone();
        thread::spawn(move || {
            loop {
                let r = read_lock.read().unwrap();
                println!("Read: {}", *r);
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
    
    thread::sleep(Duration::from_secs(1));
    
    // 写线程，可能会被“饿死”
    thread::spawn(move || {
        for i in 1..10 {
            thread::sleep(Duration::from_millis(1000));
            let mut w = rwlock.write().unwrap();
            *w += i;
            println!("Write: {}", *w);
        }
    });

    thread::sleep(Duration::from_secs(10)); // 保持程序运行一段时间
}