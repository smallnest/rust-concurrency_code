use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct BlockingQueue<T> {
    queue: Arc<Mutex<Vec<T>>>,
    cond: Condvar,
}

impl<T> BlockingQueue<T> {
    fn new() -> Self {
        BlockingQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
            cond: Condvar::new(),
        }
    }

    fn push(&self, value: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(value);
        self.cond.notify_one();
    }

    fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        loop {
            if let Some(value) = queue.pop() {
                return value;
            } else {
                queue = self.cond.wait(queue).unwrap();
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