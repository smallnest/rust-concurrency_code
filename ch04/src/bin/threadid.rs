use std::thread;
use thread_id;

fn main() {
    let handle = thread::spawn(move || {
        println!("spawned thread has id {}", thread_id::get());
    });
    
    println!("main thread has id {}", thread_id::get());
    
    handle.join().unwrap();
}