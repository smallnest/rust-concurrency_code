use std::thread;

fn main() {
    let x = 1;
    let thread_x = &x;
    
    // 不像非作用域线程，作用域线程可以借用非 'static 数据，因为作用域保证所有线程将在作用域结束时被加入。
    // 在作用域内生成的所有线程，如果没有手动加入，将在此函数返回之前自动加入。
    thread::scope(|s| {
        s.spawn(|| {
            println!("x = {}", *thread_x);
        });
    });
}


