use std::sync::{Arc,RwLock};
use std::thread;

fn main() {
    let rwlock = Arc::new(RwLock::new(0));

    let rwlock_clone = rwlock.clone();
    thread::spawn(move || {
        let _w = rwlock_clone.write().unwrap();
        panic!("Write thread panicked!");
    }).join().unwrap_err();

    // 读操作会因为毒化而失败
    let r = rwlock.read();
    println!("Read result: {:?}", r);

    //使用clear_poison可以清除毒化状态
    let _clear = rwlock.clear_poison();
    let r2 = rwlock.read().unwrap();
    println!("Read result after clear_poison: {:?}", r2);
}