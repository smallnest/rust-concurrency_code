use thread_amount::thread_amount;

use std::thread;

fn main() {
    let amount = thread_amount();

    let handle = thread::spawn(move || {
        if !amount.is_none() {
            println!("thread_amount: {}", amount.unwrap());
        }
    });

    handle.join().unwrap();
}