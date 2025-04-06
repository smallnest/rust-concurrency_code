use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    // 还没有数据，返回None
    assert!(receiver.try_iter().next().is_none());
    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();
    });
    
    // 还没有数据，返回None
    assert!(receiver.try_iter().next().is_none());
    
    // 等待两秒
    thread::sleep(Duration::from_secs(2));
    
    let mut iter = receiver.try_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    // 迭代器已经没有数据了
    assert_eq!(iter.next(), None);
}