use std::thread;
use std::time::Duration;

use ctx_thread::scope;

fn main() {
    scope(|ctx| {
        ctx.spawn(|ctx| {
            while ctx.active() {
                thread::sleep(Duration::from_secs(1));
                println!("thread run and ctx is active");
            }

            println!("thread end");
        });
    
        ctx.spawn(|ctx| {
            thread::sleep(Duration::from_secs(5));
            ctx.cancel();
        });
    }).unwrap();

    println!("main thread end");
}