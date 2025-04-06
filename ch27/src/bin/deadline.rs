#![feature(deadline_api)]
use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;

fn main() {
    let (send, recv) = mpsc::channel();

    thread::spawn(move || {
        send.send('a').unwrap();
    });

    assert_eq!(
        // 这里会阻塞，直到接收到值
        // 但是如果在400ms内没有接收到值，就会返回一个错误
        recv.recv_deadline(Instant::now() + Duration::from_millis(400)),
        Ok('a')
    );
}