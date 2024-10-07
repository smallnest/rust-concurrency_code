// 获取线程的优先级

use std::thread;
use thread_priority::*;

fn main() {
    // 使用 get_current_thread_priority
    let handle1 = thread::spawn(|| {
        let priority = get_current_thread_priority().unwrap();
        println!("Thread1 priority: {:?}", priority);
    });

    let handle2 = thread::spawn(|| {
        let priority = get_current_thread_priority().unwrap();
        println!("Thread2 priority: {:?}", priority);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();


    let priority = get_thread_priority(thread_priority::unix::thread_native_id()).unwrap();
    println!("Main thread priority: {:?}", priority);

}