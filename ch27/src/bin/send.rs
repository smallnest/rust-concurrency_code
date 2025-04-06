use std::sync::mpsc::channel;

fn main() {
    // 创建一个异步通道
    let (sender, receiver) = channel();
    let result = sender.send(42);
    println!("{:?}", result); // 发送成功，返回 Ok

    drop(receiver); // 关闭接收器

    let result = sender.send(42);
    println!("{:?}", result); // 发送失败，返回 Err
}
