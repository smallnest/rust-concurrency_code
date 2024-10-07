// 使用vec join 

use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("hello from thread {}", i);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn loop_handles() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("hello from thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}