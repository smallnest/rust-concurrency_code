use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    // 等待线程启动
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    
    loop {
        let result = cvar
            .wait_timeout(started, Duration::from_millis(100))
            .unwrap();
        // 10毫秒过去了，我们还没有收到通知。
        started = result.0;
        if *started == true {
            println!("Received the notification");
            break;
        } else {
            println!("Timeout occurred, still waiting...");
        }
    }
}
