use std::time;
use std::thread;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(2);

    pool.execute(|| {
        thread::sleep(time::Duration::from_secs(10));
    });
    pool.execute(|| {
        panic!("Panic from my custom thread!");
    });

    pool.execute(|| println!("hello"));
    pool.execute(|| println!("world"));
    pool.execute(|| println!("foo"));
    pool.execute(|| println!("bar"));

    pool.join();
    
    println!("active_count: {}", pool.active_count());
    println!("queued_count: {}", pool.queued_count());
    println!("max_count: {}", pool.max_count());
    println!("panic_count: {}", pool.panic_count()); 
     
   
}
