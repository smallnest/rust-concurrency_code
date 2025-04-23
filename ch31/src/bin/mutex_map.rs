use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个被 Arc 和 Mutex 包裹的 HashMap
    let shared_map = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    let mut handles = vec![];

    for i in 0..5 {
        // 克隆 Arc，增加引用计数，这样每个线程都有一个指向 Mutex<HashMap> 的指针
        let map_clone = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            // 获取锁，如果锁被其他线程持有，这里会阻塞
            let mut map = map_clone.lock().unwrap(); // unwrap() 在这里是为了简化，实际项目中要处理可能的毒化错误

            let key = format!("key_{}", i);
            let value = format!("value_from_thread_{}", i);
            println!("线程 {} 正在插入: {} -> {}", i, key, value);
            map.insert(key, value);

            // 锁会自动在 map 变量离开作用域时释放
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 在主线程中访问，同样需要获取锁
    let map = shared_map.lock().unwrap();
    println!("\n最终的 Map 内容:");
    for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }
    println!("Map size: {}", map.len());
}