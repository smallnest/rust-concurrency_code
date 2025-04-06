use std::sync::atomic::AtomicBool;
use std::sync::atomic::fence;
use std::sync::atomic::Ordering;

// 一个排他锁，基于自旋锁实现。
pub struct Mutex {
    flag: AtomicBool,
}

impl Mutex {
    pub fn new() -> Mutex {
        Mutex {
            flag: AtomicBool::new(false),
        }
    }

    // 获取锁
    pub fn lock(&self) {
        // 等待直到标识从false变为true
        // 自旋， CPU占用可能很高，不要用在生产环境中
        while self
            .flag
            .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {}
        // 这个屏障和`unlock`中的存储操作同步。
        // 这意味着在这个屏障之前的所有操作都必须在这个屏障之后完成。
        fence(Ordering::Acquire);
    }

    pub fn unlock(&self) {
        // 这个屏障和`lock`中的fence操作同步。
        //
        self.flag.store(false, Ordering::Release);
    }
}

fn main() {
    let mutex = Mutex::new();
    let mut data = 0;

    // 锁定互斥锁
    mutex.lock();
    // 修改数据
    data += 1;
    // 解锁互斥锁
    mutex.unlock();

    println!("数据: {}", data);
}