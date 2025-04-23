use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let shared_map = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    // --- 启动一个写线程 ---
    let map_clone_writer = Arc::clone(&shared_map);
    let writer_handle = thread::spawn(move || {
        for i in 0..3 {
            // 获取写锁
            let mut map = map_clone_writer.write().unwrap();
            let key = format!("key_{}", i);
            let value = format!("value_written_{}", i);
            println!("写线程: 插入 {} -> {}", key, value);
            map.insert(key.clone(), value);
            // 显式 drop 锁，让读线程有机会运行 (或者等作用域结束)
            drop(map);
            thread::sleep(Duration::from_millis(10)); // 模拟写入间隔
        }
    });

    // --- 启动多个读线程 ---
    let mut reader_handles = vec![];
    for i in 0..5 {
        let map_clone_reader = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                 // 获取读锁
                 let map = map_clone_reader.read().unwrap();
                 if let Some(value) = map.get("key_1") {
                     println!("读线程 {}: 读取到 key_1 -> {}", i, value);
                 } else {
                    // println!("读线程 {}: 未找到 key_1", i);
                 }
                 // 读锁在 map 离开作用域时释放
                 drop(map); // 显式释放，便于观察
                 thread::sleep(Duration::from_millis(5)); // 模拟读取间隔
            }
        });
        reader_handles.push(handle);
    }

    // 等待所有线程结束
    writer_handle.join().unwrap();
    for handle in reader_handles {
        handle.join().unwrap();
    }

    println!("\n最终 Map (主线程读取):");
    let map = shared_map.read().unwrap();
     for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }
}