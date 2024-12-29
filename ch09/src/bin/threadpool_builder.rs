use std::thread;
use threadpool;

fn main() {
    let pool = threadpool::Builder::new()
    .num_threads(8) // 设置线程池中线程的数量
    .thread_name("my-thread".into()) // 设置线程的名称
    .thread_stack_size(8_000_000) // 设置线程的栈大小, 默认为 2MB, 这里设置为 8MB
    .build();

    for i in 0..10 {
        pool.execute(|| {
            println!("Hello from my custom thread: {}!", thread::current().name().unwrap());
        });
    }
   
    pool.join();
}
