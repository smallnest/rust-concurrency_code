use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let shared_vec = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 读取线程
    let vec_clone = Arc::clone(&shared_vec);
    let handle1 = thread::spawn(move || match vec_clone.read() {
        Ok(guard) => println!("Read access: {:?}", *guard),
        Err(_) => eprintln!("RwLock read poisoned"),
    });

    // 写入线程
    let vec_clone = Arc::clone(&shared_vec);
    let handle2 = thread::spawn(move || match vec_clone.write() {
        Ok(mut guard) => guard.push(4),
        Err(_) => eprintln!("RwLock write poisoned"),
    });

    // 等待线程完成
    handle1.join().unwrap();
    handle2.join().unwrap();
}
