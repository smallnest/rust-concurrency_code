#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use tokio::runtime::Runtime;

    #[test]
    fn test_async() {
        let rt = Runtime::new().unwrap();
        let data = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        rt.block_on(async {
            for i in 0..3 {
                let data_clone = Arc::clone(&data);
                handles.push(tokio::spawn(async move {
                    let mut value = data_clone.lock().unwrap();
                    *value += i;
                    // 错误：在异步任务中阻塞
                    thread::sleep(std::time::Duration::from_millis(100));
                }));
            }

            for handle in handles {
                handle.await.unwrap();
            }
        });

        println!("Final value: {}", *data.lock().unwrap());
    }
}
