use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};
use time::Duration;

fn main() {
    let five = Arc::new(5);

    for _ in 0..10 {
        let five = Arc::clone(&five);
        thread::spawn(move || {
            println!("{five:?}");
        });
    }

    //-----------------------------------------
    let val = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let val = Arc::clone(&val);

        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::Relaxed);
            println!("{v:?}");
        });
    }

    thread::sleep(Duration::from_secs(1));
}
