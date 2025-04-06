use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

// 队列节点结构
struct Node<T> {
    value: Option<T>,            // 存储的值，None表示哨兵节点
    next: AtomicPtr<Node<T>>,    // 原子指针指向下一个节点
}

// 无锁队列结构
pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,    // 队列头部指针
    tail: AtomicPtr<Node<T>>,    // 队列尾部指针
}

impl<T> Node<T> {
    // 创建新节点
    fn new(value: Option<T>) -> *mut Self {
        let node = Box::new(Self {
            value,
            next: AtomicPtr::new(ptr::null_mut()),
        });
        Box::into_raw(node)
    }
}

impl<T> LockFreeQueue<T> {
    // 创建新队列
    pub fn new() -> Self {
        // 创建哨兵节点（不存储实际值）
        let sentinel = Node::new(None);
        
        // 初始化时头尾都指向哨兵节点
        Self {
            head: AtomicPtr::new(sentinel),
            tail: AtomicPtr::new(sentinel),
        }
    }
    
    // 入队操作
    pub fn enqueue(&self, value: T) {
        // 创建新节点
        let new_node = Node::new(Some(value));
        
        // 不断尝试将新节点添加到队列尾部
        loop {
            // 加载当前尾节点
            let tail = self.tail.load(Ordering::Acquire);
            let tail_next = unsafe { (*tail).next.load(Ordering::Acquire) };
            
            // 检查尾指针是否一致
            if tail == self.tail.load(Ordering::Acquire) {
                if tail_next.is_null() {
                    // 尾节点没有后继，尝试将新节点设为尾节点的后继
                    match unsafe { (*tail).next.compare_exchange(
                        ptr::null_mut(),
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed
                    ) } {
                        Ok(_) => {
                            // 成功链接新节点，尝试更新尾指针
                            let _ = self.tail.compare_exchange(
                                tail,
                                new_node,
                                Ordering::Release,
                                Ordering::Relaxed
                            );
                            // 无论尾指针更新成功与否，入队操作已完成
                            return;
                        }
                        Err(_) => {
                            // 其他线程改变了尾节点的next，重试
                            continue;
                        }
                    }
                } else {
                    // 尾指针落后了，帮助更新尾指针并重试
                    let _ = self.tail.compare_exchange(
                        tail,
                        tail_next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                }
            }
            // 如果尾指针已被其他线程更新，则重试整个操作
        }
    }
    
    // 出队操作
    pub fn dequeue(&self) -> Option<T> {
        loop {
            // 加载当前头节点（哨兵）和它的下一个节点
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };
            
            // 检查头指针是否一致
            if head == self.head.load(Ordering::Acquire) {
                // 队列为空的两种情况
                if head == tail {
                    // 情况1: 头尾指向同一节点
                    if next.is_null() {
                        // 队列确实为空
                        return None;
                    }
                    // 尾指针落后了，帮助更新
                    let _ = self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                } else {
                    // 队列不为空，尝试出队
                    // 首先读取值，因为节点出队后我们会释放它
                    let value = unsafe { ptr::read(&(*next).value) };
                    
                    // 尝试更新头指针
                    if self.head.compare_exchange(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    ).is_ok() {
                        // 头指针更新成功，释放原头节点
                        unsafe {
                            // 确认这是一个有效的Box，然后释放它
                            Box::from_raw(head);
                            // 提取值
                            return value;
                        }
                    }
                }
            }
            // 如果头指针已被其他线程更新，则重试
        }
    }
}

// 实现Drop来确保队列被正确清理
impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        // 出队并释放所有节点
        while self.dequeue().is_some() {}
        
        // 释放哨兵节点
        let sentinel = self.head.load(Ordering::Relaxed);
        if !sentinel.is_null() {
            unsafe {
                Box::from_raw(sentinel);
            }
        }
    }
}

// 基本测试函数
fn main() {
    use std::thread;
    use std::sync::Arc;
    
    // 创建共享队列
    let queue = Arc::new(LockFreeQueue::<i32>::new());
    let mut handles = vec![];
    
    // 多个生产者线程
    for i in 0..5 {
        let q = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let value = i * 100 + j;
                q.enqueue(value);
                println!("线程 {} 入队: {}", i, value);
                thread::sleep(std::time::Duration::from_millis(5));
            }
        });
        handles.push(handle);
    }
    
    // 多个消费者线程
    for i in 0..3 {
        let q = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for _ in 0..16 {
                match q.dequeue() {
                    Some(value) => println!("线程 {} 出队: {}", i, value),
                    None => println!("线程 {} 尝试出队: 队列为空", i),
                }
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 检查队列中是否还有剩余项
    println!("剩余项:");
    while let Some(value) = queue.dequeue() {
        println!("值: {}", value);
    }
}