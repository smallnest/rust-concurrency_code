use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    // 创建一个同步通道，缓冲区大小为1
    let (sender, receiver) = sync_channel(1);

    // 这次调用会立即返回
    sender.send(1).unwrap();

    thread::spawn(move || {
        // 这次调用会阻塞，直到前一个消息被接收
        sender.send(2).unwrap();
    });

    assert_eq!(receiver.recv().unwrap(), 1);
    assert_eq!(receiver.recv().unwrap(), 2);
}
