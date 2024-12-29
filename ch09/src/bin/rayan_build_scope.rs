use scoped_tls;
use rayon::ThreadPoolBuilder;

// 定义线程局部存储，用于存储计数器
scoped_tls::scoped_thread_local!(static COUNTER: i32);

fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let initial_value = 42;

    // 创建线程池并在作用域内共享数据
    ThreadPoolBuilder::new()
        .build_scoped(
            // 为每个线程设置初始值
            |thread| COUNTER.set(&initial_value, || thread.run()),
            // 在线程池中使用共享数据
            |pool| pool.install(|| {
                // 验证可以访问共享数据
                if COUNTER.is_set() {
                    println!("线程可以访问计数器值: {}", COUNTER.with(|&x| x));
                }
            }),
        )?;

    Ok(())
}