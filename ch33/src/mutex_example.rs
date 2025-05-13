pub struct Counter {
    count: u64, // 用于存储计数器的值
}

impl Counter {
    // 创建一个新的计数器实例，初始值为 0
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    // 将计数器的值加 1
    pub fn increment(&mut self) {
        self.count += 1;
    }

    // 获取计数器的当前值
    pub fn get(&self) -> u64 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 单线程例子
    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.get(), 0); // 初始值为 0
        counter.increment();
        assert_eq!(counter.get(), 1); // 增加后值为 1
        counter.increment();
        assert_eq!(counter.get(), 2); // 再增加后值为 2
    }

    // 多线程例子
    #[test]
    fn test_counter_multithreaded() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let counter = Arc::new(Mutex::new(Counter::new()));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut counter = counter_clone.lock().unwrap();
                counter.increment();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.lock().unwrap().get(), 10); // 最终值应该为 10
    }

    // 多线程死锁的例子
    #[test]
    fn test_counter_deadlock() {
        use std::sync::{Arc, Mutex};
        use std::thread;

        let counter1 = Arc::new(Mutex::new(Counter::new()));
        let counter2 = Arc::new(Mutex::new(Counter::new()));

        let counter1_clone = Arc::clone(&counter1);
        let counter2_clone = Arc::clone(&counter2);

        let handle1 = thread::spawn(move || {
            let _lock1 = counter1.lock().unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
            let _lock2 = counter2.lock().unwrap();
        });

        let handle2 = thread::spawn(move || {
            let _lock2 = counter2_clone.lock().unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
            let _lock1 = counter1_clone.lock().unwrap();
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}