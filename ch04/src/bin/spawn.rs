use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // 这里是新线程中执行的代码
        for i in 1..10 {
            println!("子线程中：#{}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // 主线程继续执行
    for i in 1..5 {
        println!("主线程中：#{}", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    // 等待新线程结束
    handle.join().unwrap();
}