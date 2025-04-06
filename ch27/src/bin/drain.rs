use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::thread;

fn main() {
    let (send, recv) = mpsc::channel();
    let handle = thread::spawn(move || {
        send.send(1u8).unwrap();
        send.send(2).unwrap();
        send.send(3).unwrap();
        drop(send);
    });

    // 等待发送线程结束，确保发送者被丢弃
    handle.join().unwrap();

    // 虽然发送者已被丢弃，但是通道中缓存的数据还是可以取出
    assert_eq!(Ok(1), recv.recv());
    assert_eq!(Ok(2), recv.recv());
    assert_eq!(Ok(3), recv.recv());
    assert_eq!(Err(RecvError), recv.recv());
}
