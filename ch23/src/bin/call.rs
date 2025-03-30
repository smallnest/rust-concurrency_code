use std::sync::Once;
use std::thread;
use std::time::Duration;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

// 获取缓存的值。
// 如果没有缓存值, 则调用 expensive_computation() 函数获取值,
// 并将值缓存起来, 下次调用时直接返回缓存的值。
fn get_cached_val() -> usize {
    unsafe {
        INIT.call_once(|| {
            VAL = expensive_computation();
        });
        VAL
    }
}

fn expensive_computation() -> usize {
    // 非常耗时的操作
    thread::sleep(Duration::from_secs(2));
    42
}

fn main() {
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn(move || {
            let value = get_cached_val();
            println!("线程 {:?}, 获取值: {}", thread::current().id(), value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
