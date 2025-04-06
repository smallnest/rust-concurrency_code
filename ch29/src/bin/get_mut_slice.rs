#![feature(atomic_from_mut)]
use std::sync::atomic::{AtomicI16, Ordering};

fn main() {
    // 1. 创建一个原子变量数组，并用0初始化。
    let mut some_ints = [const { AtomicI16::new(0) }; 10];

    // 2. 使用 AtomicI16::get_mut_slice() 获取一个数组可变引用。
    let view: &mut [i16] = AtomicI16::get_mut_slice(&mut some_ints);

    assert_eq!(view, [0; 10]);
    // 通过可变引用修改底层的值
    view.iter_mut()
        .enumerate()
        .for_each(|(idx, int)| *int = idx as _);

    std::thread::scope(|s| {
        // 通过原子加载操作验证底层的值
        some_ints.iter().enumerate().for_each(|(idx, int)| {
            s.spawn(move || assert_eq!(int.load(Ordering::Relaxed), idx as _));
        })
    });
}
