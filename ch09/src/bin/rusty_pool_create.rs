use rusty_pool::Builder;
use rusty_pool::ThreadPool;
use std::time::Duration;

fn main() {
    // 创建默认的 `ThreadPool` 配置，核心池大小为 CPU 数量
    let pool = ThreadPool::default();
    pool.execute(|| {
        println!("Hello from my custom thread!");
    });
    println!("name:{}, current: {}, idle: {}", pool.get_name(),
        pool.get_current_worker_count(),pool.get_idle_worker_count());


    // 创建一个默认命名的 `ThreadPool`, 核心池大小为 5，最大池大小为 50，线程空闲时间为 60 秒
    let pool2 = ThreadPool::new(5, 50, Duration::from_secs(60));
    println!("name:{}, current: {}, idle: {}", pool2.get_name(),
        pool2.get_current_worker_count(),pool2.get_idle_worker_count());

    // 创建一个自定义名称的 `ThreadPool`, 核心池大小为 5，最大池大小为 50，线程空闲时间为 60 秒
    let pool3 = ThreadPool::new_named(String::from("my_pool"), 5, 50, Duration::from_secs(60));
    println!("name:{}, current: {}, idle: {}", pool3.get_name(),
        pool3.get_current_worker_count(),pool3.get_idle_worker_count());

    // 使用 Builder 结构体创建, 核心池大小为 5，最大池大小为 50
    let pool4 = Builder::new().core_size(5).max_size(50).build();
    println!("name:{}, current: {}, idle: {}", pool4.get_name(),
        pool4.get_current_worker_count(),pool4.get_idle_worker_count());
    
    pool4.start_core_threads();
    println!("name:{}, current: {}, idle: {}", pool4.get_name(),
        pool4.get_current_worker_count(),pool4.get_idle_worker_count());
}
