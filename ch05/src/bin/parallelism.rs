use std::thread;

fn main() {
    let parallelism = thread::available_parallelism().unwrap();
    println!("当前系统的并发度: {}", parallelism);

    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    } else {
        println!("num_threads: not supported");
    }

    if let Some(count) = thread_amount::thread_amount() {
        println!("thread_amount: {}", count);
    } else {
        println!("thread_amount: not supported");
    }
    
    let count = num_cpus::get();
    println!("num_cpus: {}", count);
    
}