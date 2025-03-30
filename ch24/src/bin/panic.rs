use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(""), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = "时代变了";
        // 通知所有的等待线程，变量已经改变
        cvar.notify_all();
    });

    for _ in 0..5 {
        let pair2 = Arc::clone(&pair);
        thread::spawn(move || {
            // 等待直到上面的线程改变了值
            let (_, cvar) = &*pair2;
            let lock2 = Mutex::new("");
            let mut started = lock2.lock().unwrap();
            // 等待变量直到它被设置
            while started.is_empty() {
                started = cvar.wait(started).unwrap();
            }
        });
    }

    // 等待直到上面的线程改变了值
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // 等待变量直到它被设置
    while started.is_empty() {
        started = cvar.wait(started).unwrap();
    }
}
