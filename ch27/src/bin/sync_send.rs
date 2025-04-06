use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    // 创建一个同步通道，缓冲区大小为0
    let (sync_sender, receiver) = sync_channel(0);

    thread::spawn(move || {
        println!("sending message...");
        sync_sender.send(1).unwrap(); // 线程会被阻塞，直到接收者准备好接收数据
        println!("...message received!");
    });

    let msg = receiver.recv().unwrap();
    assert_eq!(1, msg);
}
