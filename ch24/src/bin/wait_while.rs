use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(true), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        cvar.notify_one();
        let mut pending = lock.lock().unwrap();
        *pending = false;
        cvar.notify_one();
    });

    // 等待线程启动
    let (lock, cvar) = &*pair;

    let _guard = cvar
        .wait_while(lock.lock().unwrap(), |pending| {
            println!("Waiting for checking condition: {}", *pending);
            *pending // 如果条件为true，则继续等待
        })
        .unwrap();
}
