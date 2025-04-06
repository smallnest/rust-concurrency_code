use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    // 启动一个线程来发送一个消息，延迟2秒
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send("Hello").unwrap();
    });

    // 这里的超时是1秒
    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(msg) => println!("Received: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

