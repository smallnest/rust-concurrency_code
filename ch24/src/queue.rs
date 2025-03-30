use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

// 线程安全队列结构
pub struct Queue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> Queue<T> {
    // 创建一个新的空队列
    pub fn new() -> Self {
        Queue {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    // 向队列尾部添加元素
    pub fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);

        if queue.len() == 1 {
            // 如果队列之前为空，通知等待中的线程
            self.condvar.notify_one();
        }
    }

    // 从队列头部移除并返回元素，如果队列为空则阻塞等待
    pub fn pop(&self) -> T {
        let mut queue = self.queue.lock().unwrap();
        
        // 当队列为空时等待
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        
        // 当队列不为空时，取出头部元素
        queue.pop_front().unwrap()
    }

    // 尝试从队列头部移除并返回元素，如果队列为空则立即返回None
    pub fn try_pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    // 返回队列中元素的数量
    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }

    // 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread; 

    #[test]
    fn test_basic_operations() {
        let queue = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        
        queue.push(1);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);
        
        assert_eq!(queue.try_pop(), Some(1));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_multiple_threads() {
        let queue = std::sync::Arc::new(Queue::new());
        let queue_clone = queue.clone();

        let producer = thread::spawn(move || {
            for i in 0..5 {
                queue_clone.push(i);
                thread::sleep(Duration::from_millis(10));
            }
        });

        let consumer = thread::spawn(move || {
            let mut sum = 0;
            for _ in 0..5 {
                sum += queue.pop();
            }
            sum
        });

        producer.join().unwrap();
        assert_eq!(consumer.join().unwrap(), 10);
    }

    #[test]
    fn test_try_pop_empty() {
        let queue = Queue::<i32>::new();
        assert_eq!(queue.try_pop(), None);
    }

    #[test]
    fn test_fifo_order() {
        let queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.pop(), 2);
        assert_eq!(queue.pop(), 3);
    }
}