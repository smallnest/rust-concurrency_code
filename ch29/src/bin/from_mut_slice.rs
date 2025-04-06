#![feature(atomic_from_mut)]
use std::sync::atomic::{AtomicI16, Ordering};

fn main() {
    // 一个普通的 i16 数组
    let mut some_ints = [0; 10];
    // 使用AtomicI16::get_mut_slice() 获取一个AtomicI16数组引用。
    let a = &*AtomicI16::from_mut_slice(&mut some_ints);
    std::thread::scope(|s| {
        for i in 0..a.len() {
            // 通过原子操作设置底层的值
            s.spawn(move || a[i].store(i as _, Ordering::Relaxed));
        }
    });


    // 通过非原子访问的方式访问底层的值
    for (i, n) in some_ints.into_iter().enumerate() {
        assert_eq!(i, n as usize);
    }
}
