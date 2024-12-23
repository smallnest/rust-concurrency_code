use go_spawn::{go, join};
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

fn main() {

    let counter = Arc::new(AtomicI64::new(0));
    let counter_cloned = counter.clone();

    // 启动一个线程，通过 move 捕获值。
    go! {
        for _ in 0..100 {
            counter_cloned.fetch_add(1, Ordering::SeqCst);
        }
    }


    // 等待所有线程结束
    // Join the most recent thread spawned by `go_spawn` that has not yet been joined.

    // 等待最近一个由 `go_spawn` 启动的线程结束。
    assert!(join!().is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}