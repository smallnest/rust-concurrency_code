use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (send, recv) = channel();

    thread::spawn(move || {
        // 发送三个值, 然后send会被丢弃
        send.send(1).unwrap();
        send.send(2).unwrap();
        send.send(3).unwrap();
    });

    for n in recv {
        // 迭代器会阻塞，直到接收到值
        println!("Received: {}", n);
    }
}