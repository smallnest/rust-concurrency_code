use std::sync::{Arc, Mutex};
use std::thread;

fn main(){
    let mutex = Arc::new(Mutex::new(0));
    let c_mutex = Arc::clone(&mutex);
    
    thread::spawn(move || {
        let mut lock = c_mutex.try_lock();
        if let Ok(ref mut mutex) = lock {
            **mutex = 10;
        } else {
            println!("try_lock failed");
        }
    }).join().expect("thread::spawn failed");
    assert_eq!(*mutex.lock().unwrap(), 10);
}
