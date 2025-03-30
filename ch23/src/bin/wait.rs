#![feature(once_wait)]

use std::sync::Once;
use std::{panic, thread};
use std::time::Duration;

static READY: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(|| {
        READY.wait();
        println!("everything is ready");
    });
    
    let handle2 = thread::spawn(|| {
        READY.call_once(|| panic!("poisoned"));
    });

    
    println!("thread2: {}", handle2.join().is_err());
    println!("thread1: {}", handle1.join().is_err());

}