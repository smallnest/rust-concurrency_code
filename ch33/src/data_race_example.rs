#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_data_race() {
        let data = Arc::new(Mutex::new(42));

        let data_clone1 = Arc::clone(&data);
        let t1 = thread::spawn(move || {
            let mut value = data_clone1.lock().unwrap();
            *value = 100; // 线程1安全修改数据
        });

        let data_clone2 = Arc::clone(&data);
        let t2 = thread::spawn(move || {
            let mut value = data_clone2.lock().unwrap();
            *value = 200; // 线程2安全修改数据
        });

        t1.join().unwrap();
        t2.join().unwrap();

        // 输出可能是100或200，取决于线程调度，但不会有未定义行为。
        println!("Data: {}", *data.lock().unwrap());
    }
}
