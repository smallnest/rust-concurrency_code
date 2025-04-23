use evmap::{ReadHandle, WriteHandle};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个新的evmap，分别获取读句柄和写句柄
    let (r_handle, mut w_handle): (ReadHandle<String, i32>, WriteHandle<String, i32>) = evmap::new();

    // 在一个单独的线程中进行写操作
    let write_thread = thread::spawn(move || {
        // 添加一些键值对
        w_handle.insert("key1".to_string(), 10);
        w_handle.insert("key2".to_string(), 20);
        w_handle.insert("key3".to_string(), 30);
        
        // 必须调用refresh()使更改对读者可见
        w_handle.refresh();
        println!("Initial data written and refreshed");
        
        // 等待一段时间
        thread::sleep(Duration::from_millis(500));
        
        // 更新一个值
        w_handle.update("key1".to_string(), 15);
        // 删除一个键
        w_handle.empty("key3".to_string());
        
        // 再次refresh使更改可见
        w_handle.refresh();
        println!("Updates applied and refreshed");
        
        // 保持写句柄活着
        thread::sleep(Duration::from_secs(2));
    });

    // 主线程作为读者
    thread::sleep(Duration::from_millis(100)); // 等待写线程完成初始写入
    
    // 获取一个读视图
    let map = r_handle.clone();
    
    // 读取并打印值
    println!("First read:");
    if let Some(values) = map.get("key1") {
        println!("key1: {:?}", values);
    }
    if let Some(values) = map.get("key2") {
        println!("key2: {:?}", values);
    }
    if let Some(values) = map.get("key3") {
        println!("key3: {:?}", values);
    }
    
    // 等待更新
    thread::sleep(Duration::from_secs(1));
    
    // 重新获取读视图（会看到新的更改）
    println!("\nSecond read after updates:");
    if let Some(values) = map.get("key1") {
        println!("key1: {:?}", values);
    } else {
        println!("key1: not found");
    }
    if let Some(value) = map.get_one("key2") {
        println!("key2: {:?}", *value);
    } else {
        println!("key2: not found");
    }
    if let Some(values) = map.get("key3") {
        println!("key3: {:?}", values);
    } else {
        println!("key3: not found");
    }
    
    // 等待写线程完成
    write_thread.join().unwrap();
}