use std::thread;

#[macro_export]
macro_rules! join_all {
    // 1. 空参数情况
    () => {};

    // 2. Vec<JoinHandle> 情况 - 使用特定的类型标记来匹配
    ($handles:expr) => {{
        let handles = $handles;
        // 如果是 Vec，这个转换会成功
        let _: ::std::vec::Vec<_> = handles;
        for handle in handles {
            if let Ok(result) = handle.join() {
                result
            }
        }
    }};

    // 3. 单个 JoinHandle 情况
    ($handle:expr) => {{
        if let Ok(result) = $handle.join() {
            result
        }
    }};

    // 4. 逗号分隔的 JoinHandle 列表情况
    ($($handle:expr),+ $(,)?) => {
        $(
            if let Ok(result) = $handle.join() {
                result
            }
        )+
    };
}

fn main() {
    let handle1 = thread::spawn(|| println!("Hello from thread 1"));
    let handle2 = thread::spawn(|| println!("Hello from thread 2"));
    let handle3 = thread::spawn(|| println!("Hello from thread 3"));
    let handle4 = thread::spawn(|| println!("Hello from thread 4"));

    join_all!(handle1); // 单个 JoinHandle

    join_all!(handle1, handle2); // 多个 JoinHandle

    let handles = vec![handle3, handle4];
    join_all!(handles); // Vec<JoinHandle>

    join_all!(); // 空参数

    let handles2 = (0..2).map(|i| thread::spawn(move|| println!("Hello from thread {}", i))).collect::<Vec<_>>();
    join_all!(handles2); // 使用迭代器和 collect

    join_all!(thread::spawn(|| println!("Hello from one thread"))); //单个线程
}