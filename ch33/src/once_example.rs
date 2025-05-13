#[allow(static_mut_refs)]


#[cfg(test)]
mod tests {
    use std::sync::{Arc, Once};
    use std::thread;

    #[test]
    fn test_once() {
        static INIT: Once = Once::new();
        static mut GLOBAL_VALUE: Option<Arc<i32>> = None;

        // 定义一个可能被多个线程调用的函数
        fn get_global_value() {
            INIT.call_once(|| {
                println!(
                    "Thread {:?}: Starting initialization",
                    thread::current().id()
                );
                unsafe {
                    // 错误：初始化逻辑中再次调用 get_global_value
                    if GLOBAL_VALUE.is_none() {
                        // 模拟复杂逻辑，错误地重新触发初始化
                        get_global_value();
                    }
                    GLOBAL_VALUE = Some(Arc::new(42));
                }
                println!(
                    "Thread {:?}: Initialization complete",
                    thread::current().id()
                );
            });
        }

        let mut handles = vec![];

        for _ in 0..3 {
            let handle = thread::spawn(|| {
                get_global_value();
                unsafe {
                    println!(
                        "Thread {:?}: Global value = {:?}",
                        thread::current().id(),
                        GLOBAL_VALUE.as_ref()
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
