use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell; // 需要 UnsafeCell 来包裹数据，因为锁内部需要可变访问

// 一个简单的自旋锁结构体
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>, // 数据被 UnsafeCell 包裹
}

// 需要手动实现 Send 和 Sync，因为 UnsafeCell 不是 Sync 的
// 我们断言锁的逻辑保证了线程安全
unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false), // false 表示未锁定
            data: UnsafeCell::new(data),
        }
    }

    // 获取锁，返回一个 RAII 守护对象
    pub fn lock(&self) -> SpinLockGuard<T> {
        // 循环尝试将 false (未锁定) 交换为 true (锁定)
        // Acquire 顺序: 确保成功获取锁之后的操作，发生在获取锁之后，
        // 并且能看到之前释放锁的线程的所有写入。
        // Relaxed 失败顺序: 如果 CAS 失败，不需要同步保证。
        while self.locked.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // 提示 CPU 我们在自旋等待，可能降低功耗或让出超线程资源
            std::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }
    // 解锁操作由 SpinLockGuard 的 Drop 实现自动处理
}

// RAII 守护对象，用于自动解锁
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// 实现 Deref 和 DerefMut，让守护对象可以像 T 的引用一样使用
impl<'a, T> std::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: 我们持有锁，保证了对数据的独占访问权
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: 我们持有锁，保证了对数据的独占访问权
        unsafe { &mut *self.lock.data.get() }
    }
}

// 实现 Drop trait，在守护对象离开作用域时自动释放锁
impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        // Release 顺序: 确保在解锁之前对受保护数据的所有修改，
        // 对下一个成功获取锁的线程可见。
        self.lock.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let lock = SpinLock::new(0);
    let mut guard = lock.lock();
    *guard += 1;
    drop(guard); // 锁在这里被释放
}