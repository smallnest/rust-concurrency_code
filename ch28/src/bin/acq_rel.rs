use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个共享的原子标志和数据
    let flag = Arc::new(AtomicBool::new(false));
    let mut data = 0;
    
    // 克隆标志用于生产者线程
    let flag_clone = Arc::clone(&flag);
    
    // 启动生产者线程
    let producer = thread::spawn(move || {
        // 模拟一些数据准备工作
        println!("生产者: 正在准备数据...");
        thread::sleep(Duration::from_millis(50));
        
        // 在标志设置前修改数据
        data = 42;
        
        // 使用 Release 序保证这之前的所有写入都对设置标志的线程可见
        println!("生产者: 数据已准备好，设置标志...");
        flag_clone.store(true, Ordering::Release);
    });
    
    // 启动消费者线程
    let consumer = thread::spawn(move || {
        // 不断检查标志是否被设置
        println!("消费者: 等待数据准备就绪...");
        
        // 自旋等待标志变为 true
        while !flag.load(Ordering::Acquire) {
            // 使用 Acquire 保证标志加载后的所有读取都能看到标志设置前的写入
            thread::yield_now(); // 让出CPU时间
        }
        
        // 一旦标志被设置，保证能看到生产者设置标志之前对数据的修改
        println!("消费者: 标志被设置，数据 = {}", data);
    });
    
    // 等待两个线程完成
    producer.join().unwrap();
    consumer.join().unwrap();
}