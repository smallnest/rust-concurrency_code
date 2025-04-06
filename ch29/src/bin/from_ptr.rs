use std::sync::atomic::{self, AtomicI16};
fn main() {
    // 得到一个指向 i16 的原始指针
    let ptr: *mut i16 = Box::into_raw(Box::new(0));

    // 这里我们假设 ptr 是一个有效的(对齐的)指针，并且它指向一个 i16 的值
    assert!(ptr.cast::<AtomicI16>().is_aligned());
    {
        // 基于原始指针创建一个原子引用
        let atomic = unsafe { AtomicI16::from_ptr(ptr) };

        // 使用原子操作
        atomic.store(1, atomic::Ordering::Relaxed);
    }

    // 非原子地访问 `ptr` 后面的值是安全的，因为上面的块中原子的引用结束了它的生命周期
    assert_eq!(unsafe { *ptr }, 1);

    // 丢弃原始指针
    unsafe { drop(Box::from_raw(ptr)) }
}
