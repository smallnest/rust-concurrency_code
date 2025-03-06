use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // log 写入到控制台
    simple_logger::init().unwrap();

    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = Arc::clone(&mutex);

    let handle1 = thread::spawn(move || {
        {
            let mut v = c_mutex.lock().unwrap();
            log::info!("Thread1 locked");
            *v = 10;
        }
        log::info!("Thread1 unlocked");
        thread::sleep(std::time::Duration::from_secs(10));
        log::info!("Thread1 finished");
    });

    let c_mutex = Arc::clone(&mutex);
    let handle2 = thread::spawn(move || {
        let mut v = c_mutex.lock().unwrap();
        log::info!("Thread2 locked");
        *v = 10;
        drop(v);
        log::info!("Thread2 unlocked");
        thread::sleep(std::time::Duration::from_secs(10));
        log::info!("Thread2 finished");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    assert_eq!(*mutex.lock().unwrap(), 10);
}
