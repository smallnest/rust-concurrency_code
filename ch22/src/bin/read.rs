use std::{sync::RwLock, thread};
fn main() {
    let lock = RwLock::new(5);

    thread::scope(|s| {
        s.spawn(|| {
            let mut w = lock.write().unwrap();
            *w = 6;
        });
        s.spawn(|| {
            let r = lock.read().unwrap();
            println!("r = {}", r);
        });
        s.spawn(|| {
            let r = lock.read().unwrap();
            println!("r = {}", r);
        });
    });

 
    println!("lock = {:?}", lock);
}