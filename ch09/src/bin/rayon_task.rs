use rayon;
use rayon::ThreadPoolBuilder;
use std::sync::{Arc, Mutex};

fn main() {
    // 创建一个自定义线程池，包含 4 个线程
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    // 共享计数器，模拟跨线程的共享状态
    let counter = Arc::new(Mutex::new(0));
    let counter2 = Arc::clone(&counter);

    // 创建一个闭包作为任务，修改共享的计数器
    let op = move || {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        println!("Thread {:?} processed the task, counter: {}", std::thread::current().id(), *counter);
    };

    // 直接使用 `spawn` 启动任务
    pool.spawn(move || {
        println!("\nUsing spawn method");
        op(); // 使用闭包
    });

    // 直接使用 `scope` 启动多个任务（确保所有任务完成后再返回）
    pool.scope(|s| {
        println!("\nUsing scope method");
        s.spawn(|_| println!("spawn a task")); // 使用 spawn 启动任务
        s.spawn(|_| println!("spawn another task")); // 启动另一个任务
    });

    // 直接使用 `scope_fifo` 启动多个任务（先发先执行，任务按照提交顺序执行）
    pool.scope_fifo(|s| {
        println!("\nUsing scope_fifo method");
        s.spawn_fifo(|_| println!("spawn task#3"));
        s.spawn_fifo(|_| println!("spawn task#4"));
    });

    // 直接使用 `in_place_scope` 启动任务（任务在同一线程内执行，不会并行）
    pool.in_place_scope(|s| {
        println!("\nUsing in_place_scope method");
        s.spawn(|_| println!("spawn task#5"));
    });

    // 直接使用 `in_place_scope_fifo` 启动任务（先发先执行，但在同一线程内顺序执行）
    pool.in_place_scope_fifo(|s| {
        println!("\nUsing in_place_scope_fifo method");
        s.spawn_fifo(|_| println!("spawn task#6"));
    });

    // 使用 `spawn_fifo` 启动任务，确保任务按提交顺序执行
    pool.spawn_fifo(|| {
        println!("\nUsing spawn_fifo method");
    });

    // 使用 `spawn_broadcast` 广播任务到所有线程
    pool.spawn_broadcast(|_| {
        println!("\nUsing spawn_broadcast method");
    });

    // 使用 `broadcast` 方法广播任务到所有线程
    pool.broadcast(|_| {
        println!("\nUsing broadcast method");
    });

    // 等待所有任务执行完毕
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 输出最终计数器的值
    println!("\nFinal counter value: {}", *counter2.lock().unwrap());
}
