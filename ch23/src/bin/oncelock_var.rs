use std::sync::OnceLock;
use std::thread;
use std::sync::Arc;

fn process_data() -> i32 {
    // 模拟耗时的数据处理
    println!("进行耗时的数据处理...");
    42
}

fn main() {
    let shared_data: OnceLock<Arc<i32>> = OnceLock::new();

    thread::scope(|s| {
        for _ in 0..5 {
            let shared_data_ref = &shared_data; // 创建引用
            s.spawn(|| {
                let data = shared_data_ref.get_or_init(|| {
                    println!("初始化共享数据");
                    Arc::new(process_data())
                });
                println!("线程 {:?}: 获取共享数据 {}", thread::current().id(), data);
            });
        }
    });
}