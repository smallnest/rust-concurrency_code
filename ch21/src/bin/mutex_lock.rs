use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = Arc::clone(&mutex);

    let handle = thread::spawn(move || {
        let mut v = c_mutex.lock().unwrap();
        *v = 10;
    });

    handle.join().unwrap();
    
    assert_eq!(*mutex.lock().unwrap(), 10);
}
