use std::thread;
use std::sync::Mutex;

fn main() {
    test_get_mut();
}

fn test_get_mut() {
    let mut mutex = Mutex::new(0);
    *mutex.get_mut().unwrap() = 10;
    assert_eq!(*mutex.lock().unwrap(), 10);
}