use core::panic;
use std::thread;

fn main() {
    thread::scope(|s| {
        let handle1 = s.spawn(|| {
            print!("hello from the first scoped thread");
        });
        let handle2 = s.spawn(|| {
            println!("hello from the second scoped thread");
            panic!("panic from the second scoped thread");
        });

        match handle1.join() {
            Ok(_) => println!("first scoped thread finished successfully"),
            Err(_) => println!("first scoped thread panicked"),
        }

        match handle2.join() {
            Ok(_) => println!("second scoped thread finished successfully"),
            Err(_) => println!("second scoped thread panicked"),
        }
    });

    println!("hello from the main thread");
    
}

