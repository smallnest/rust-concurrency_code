#![feature(atomic_from_mut)]
use std::sync::atomic::{AtomicI16, Ordering};

fn main() {
    let mut some_int = 123;
    // 基于some_int创建一个原子引用
    // 这里我们假设 some_int 是一个有效的(对齐的)指针，并且它指向一个 i16 的值
    let a = AtomicI16::from_mut(&mut some_int);
    a.store(100, Ordering::Relaxed);
    assert_eq!(some_int, 100);
}
