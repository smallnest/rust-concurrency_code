use std::sync::{Arc, Mutex};
use std::thread;

struct BlockingQueue<T> {
    queue: Arc<Mutex<Vec<T>>>,
    available: std::sync::atomic::AtomicBool,
    thread: thread::Thread,
}

impl<T> BlockingQueue<T> {
    fn new() -> Self {
        BlockingQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
            available: std::sync::atomic::AtomicBool::new(false),
            thread: thread::current(),
        }
    }

    fn push(&self, value: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(value);
        self.available.store(true, std::sync::atomic::Ordering::SeqCst);
        self.thread.unpark();//唤醒等待的线程
    }

    fn pop(&self) -> T {
        loop {
            let mut queue = self.queue.lock().unwrap();
            if let Some(value) = queue.pop() {
                return value;
            } else {
                drop(queue); // 释放锁，避免死锁
                self.available.store(false, std::sync::atomic::Ordering::SeqCst);
                thread::park();//没有数据，挂起当前线程
            }
        }
    }
}

fn main() {
    let queue = Arc::new(BlockingQueue::new());
    let queue_clone = queue.clone();

    thread::spawn(move || {
        for i in 0..10 {
            queue_clone.push(i);
            println!("Pushed: {}", i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    for _ in 0..10 {
        let value = queue.pop();
        println!("Popped: {}", value);
    }
}