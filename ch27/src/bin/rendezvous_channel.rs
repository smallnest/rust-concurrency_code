use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    // 创建一个同步通道，缓冲区大小为1
    let (sender, receiver) = sync_channel(0);

    thread::spawn(move || {
        // 这次调用会阻塞，直到消费者准备接收数据
        sender.send(42).unwrap();
    });

    // 准备接收数据，生产者不会阻塞了
    assert_eq!(receiver.recv().unwrap(), 42);
}
