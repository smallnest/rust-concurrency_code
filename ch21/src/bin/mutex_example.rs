use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个共享的计数器，包裹在Arc和Mutex中
    let counter = Arc::new(Mutex::new(0));

    // 准备多个线程，每个线程对计数器加1
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread incremented counter to: {}", *num);
        });

        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 最终计数器的值
    println!("Final counter value: {}", *counter.lock().unwrap());
}
