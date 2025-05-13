
use shuttle::sync::{Arc, Mutex};
use shuttle::thread;

#[test]
fn test_mutex() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let lock = Arc::new(Mutex::new(0u64));
    let lock2 = lock.clone();

    thread::spawn(move || {
        *lock.lock().unwrap() = 1;
    });

    assert_eq!(0, *lock2.lock().unwrap());
}

#[test]
fn test_shuttle() {
    shuttle::check_random(|| {
        let lock = Arc::new(Mutex::new(0u64));
        let lock2 = lock.clone();
    
        thread::spawn(move || {
            *lock.lock().unwrap() = 1;
        });
    
        assert_eq!(0, *lock2.lock().unwrap());
    }, 100);
    
}
