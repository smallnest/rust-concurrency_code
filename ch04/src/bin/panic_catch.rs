use std::panic;
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let result = panic::catch_unwind(|| {
            println!("Thread running...");
            panic!("Thread panicked!");
        });

        match result {
            Ok(_) => println!("Thread completed successfully."),
            Err(_) => println!("Thread caught panic and handled it!"),
        }
    });

    match handle.join() {
        Ok(_) => println!("Main: Thread joined successfully."),
        Err(_) => println!("Main: Thread panicked!"),
    }
}
