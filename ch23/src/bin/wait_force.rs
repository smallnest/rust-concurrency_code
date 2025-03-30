#![feature(once_wait)]

use std::sync::Once;
use std::{panic, thread};
use std::time::Duration;

static READY: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(|| {
        READY.wait_force();
        println!("everything is ready");
    });
    
    let handle2 = thread::spawn(|| {
        READY.call_once(|| panic!("poisoned"));
    });

    let handle3 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        READY.call_once_force(|_| println!("call_once_forced"));
    });
    
    println!("thread2: {}", handle2.join().is_err());
    println!("thread1: {}", handle1.join().is_err());
    println!("thread3: {}", handle3.join().is_err());

}