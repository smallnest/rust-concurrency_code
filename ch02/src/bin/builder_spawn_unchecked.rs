#![feature(thread_spawn_unchecked)]

use std::thread;

fn main() {
    let builder = thread::Builder::new();

    let x = 1;
    let thread_x = &x;

    let handler = unsafe {
        builder
            .spawn_unchecked(move || {
                println!("x = {}", *thread_x);
            })
            .unwrap()
    };

    // 调用者必须确保调用 `join()`，否则如果 `x` 在线程闭包执行之前被释放，
    // 可能会访问到已释放的内存！
    handler.join().unwrap();
}
