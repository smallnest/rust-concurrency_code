// 使用papaya的HashMap
use papaya::HashMap;

fn main() {
    // 从多个线程使用map
    let map = HashMap::new();
    std::thread::scope(|s| {
        // 插入一些值
        s.spawn(|| {
            let map = map.pin();
            for i in 'A'..='Z' {
                map.insert(i, 1);
            }
        });

        // 删除这些值
        s.spawn(|| {
            let map = map.pin();
            for i in 'A'..='Z' {
                map.remove(&i);
            }
        });

        // 读取这些值
        s.spawn(|| {
            for (key, value) in map.pin().iter() {
                println!("{key}: {value}");
            }
        });
    });
}
