use fast_threadpool::{ThreadPool, ThreadPoolConfig};
use std::time::Duration;
use std::thread;

fn main() {
    let threadpool = ThreadPool::start(ThreadPoolConfig::default(), ()).into_sync_handler();

    let _ = threadpool.execute(|_| {
        println!("Hello from my custom thread!");
    });

    match threadpool.launch(|_| {
        thread::sleep(Duration::from_secs(10));
        println!("Hello from my custom sleep thread!");
    }) {
        Ok(join_handler) => {
            println!("Task launched successfully!");
            join_handler.join().unwrap();
        },
        Err(e) => eprintln!("Failed to launch task: {:?}", e),
    }
}