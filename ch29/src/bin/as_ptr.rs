use std::sync::atomic::{AtomicI16, Ordering};

unsafe fn my_atomic_op(arg: *mut i16) {
    // 方法1：直接使用 Rust 的原子操作
    unsafe{
        let atomic_ref = &*(arg as *mut AtomicI16);
        atomic_ref.fetch_add(1, Ordering::SeqCst);
    }

    // 方法2：手动修改内存（不推荐，非原子操作）
    // unsafe {
    //     *arg = *arg + 1;  // 警告：这不是原子操作！
    // }
}



fn main() {
    let atomic = AtomicI16::new(1);

    // 安全性： 这里我们假设 `my_atomic_op` 是一个原子操作
    unsafe {
        my_atomic_op(atomic.as_ptr());
    }

    println!("Atomic value: {}", atomic.load(Ordering::SeqCst)); // 输出：Atomic value: 2
}
