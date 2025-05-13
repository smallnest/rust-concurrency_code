#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_panic() {
        let data = Arc::new(Mutex::new(vec![]));
        let mut handles = vec![];

        for i in 0..3 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let mut vec = data_clone.lock().unwrap();
                vec.push(i);
                if i == 1 {
                    panic!("Thread {} panicked!", i); // 模拟 panic
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join(); // 忽略 panic
        }

        match data.lock() {
            Ok(vec) => println!("Final data: {:?}", *vec), // 数据不一致
            Err(err) => println!("Failed to lock mutex: {:?}, data: {:?}", err, err.get_ref()), // 数据不一致
        }
    }
}
