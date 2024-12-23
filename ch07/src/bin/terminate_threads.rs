use terminate_thread::Thread;

fn main() {
    let thr = Thread::spawn(|| loop {
        // infinite loop in this thread
        println!("loop run");
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("loop end"); // 这行代码不会被执行
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
    thr.terminate() // 手工终止线程
}