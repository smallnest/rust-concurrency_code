use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // 创建一个异步通道
    let (sender, receiver) = channel();

    // 启动一个线程来发送一个值
    thread::spawn(move || {
        sender.send(42).unwrap();
    });

    // 做一些其他的工作
    println!("Doing some work in the main thread...");

    // 打印接收值
    // 注意：在这里我们会阻塞，直到接收到值
    println!("{:?}", receiver.recv().unwrap());
}
