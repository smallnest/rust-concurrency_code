use rusty_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建默认的 `ThreadPool` 配置，核心池大小为 CPU 数量
    let pool = ThreadPool::default();
    pool.execute(|| {
        thread::sleep(Duration::from_secs(5));
        println!("Hello from my custom thread!");
    });
    println!(
        "name:{}, current: {}, idle: {}",
        pool.get_name(),
        pool.get_current_worker_count(),
        pool.get_idle_worker_count()
    );
    pool.join();
    println!(
        "name:{}, current: {}, idle: {}",
        pool.get_name(),
        pool.get_current_worker_count(),
        pool.get_idle_worker_count()
    );

    pool.join_timeout(Duration::from_secs(10));
}
