use rusty_pool::ThreadPool;
use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
};
use std::thread;
use std::time::Duration;

fn main() {
    let pool = ThreadPool::new(5, 50, Duration::from_secs(60));

    // 方式1： execute提交任务就直接返回
    pool.execute(|| {
        thread::sleep(Duration::from_secs(5));
        println!("Hello from my custom thread!");
    });

    // 方式2：evaluate 生成一个 JoinHandler, 然后可以调用await_complete等待完成
    let handle = pool.evaluate(|| {
        return 4;
    });
    let result = handle.await_complete();
    println!("result: {}", result);

    // 方式3：launch 启动一个任务，返回一个JoinHandler, 然后可以调用join等待完成
    async fn some_async_fn(x: i32, y: i32) -> i32 {
        x + y
    }

    async fn other_async_fn(x: i32, y: i32) -> i32 {
        x - y
    }

    // 方式3：简单的使用complete提交异步任务到一个worker上, 然后调用await_complete等待完成
    let handle = pool.complete(async {
        let a = some_async_fn(4, 6).await; // 10
        let b = some_async_fn(a, 3).await; // 13
        let c = other_async_fn(b, a).await; // 3
        some_async_fn(c, 5).await // 8
    });
    println!("result: {}", handle.await_complete());

  
    // 方式4：spawn future，创建一个waker，如果准备好以便继续执行，它会自动重新提交到线程池，这需要默认启用的“async”功能
    let count = Arc::new(AtomicI32::new(0));
    let clone = count.clone();
    pool.spawn(async move {
        let a = some_async_fn(3, 6).await; // 9
        let b = other_async_fn(a, 4).await; // 5
        let c = some_async_fn(b, 7).await; // 12
        clone.fetch_add(c, Ordering::SeqCst);
    });
    pool.join();
    println!("result: {}", count.load(Ordering::SeqCst)); // 12
}