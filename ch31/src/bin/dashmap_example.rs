// 需要在 Cargo.toml 中添加依赖: dashmap = "5.5" (请使用最新版本)
use dashmap::DashMap; // 注意是 DashMap，不是 HashMap
use std::sync::Arc;
use std::thread;

fn main() {
    // DashMap 天然支持并发，内部处理了同步，通常直接用 Arc 包裹即可共享
    let shared_map = Arc::new(DashMap::<String, String>::new());

    let mut handles = vec![];

    for i in 0..10 { // 增加线程数，更能体现优势
        let map_clone = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let key = format!("key_{}", i % 5); // 制造一些 key 的碰撞，测试并发写同一分片
            let value = format!("value_from_thread_{}", i);

            // DashMap 的 API 和 HashMap 很像，但不需要手动 .lock()
            println!("线程 {} 正在操作 key: {}", i, key);
            map_clone.insert(key.clone(), value.clone());

            // 读取也类似
            if let Some(entry) = map_clone.get(&key) {
                // println!("线程 {} 读取到: {} -> {}", i, key, entry.value());
            } else {
                 println!("线程 {} 未找到 {}", i, key); // 可能因为其他线程正在写
            }

             // DashMap 还提供了一些原子操作，比如 entry API，更灵活
             map_clone.entry(format!("entry_key_{}", i)).or_insert_with(|| format!("default_val_{}", i));

        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n最终的 DashMap 内容 (部分展示):");
    // 遍历 DashMap 没有 RwLock/Mutex 那么直接，但也很方便
    let mut count = 0;
    for entry in shared_map.iter() {
        if count < 10 { // 只打印前 10 个，避免刷屏
             println!("{} -> {}", entry.key(), entry.value());
             count += 1;
        } else {
            break;
        }
    }
     println!("DashMap size: {}", shared_map.len());
}