use send_wrapper::SendWrapper; 
use std::rc::Rc; 
use std::ops::Deref; 
use std::thread; 
use std::sync::mpsc::channel; 

fn main() {
    let wrapped_value = SendWrapper::new(Rc::new(42)); // 创建一个新的 SendWrapper 包装 Rc

    let (sender, receiver) = channel(); // 创建一个 mpsc 通道

    let _t = thread::spawn(move || { // 创建一个新的线程
        sender.send(wrapped_value).unwrap(); // 在新线程中发送 wrapped_value
    });

    let wrapped_value = receiver.recv().unwrap(); // 在主线程中接收 wrapped_value

    let value = wrapped_value.deref(); // 解引用 wrapped_value
    println!("received from the main thread: {}", value); 
}