#![feature(mpmc_channel)]

use std::sync::mpmc::channel;
use std::thread;

fn main() {
    thread::scope(|s| {
        // 创建一个共享的通道，可以从多个线程发送
        // 其中 tx 是发送半部分（tx 用于传输），rx 是接收半部分（rx 用于接收）。
        let (tx, rx) = channel();
        for i in 0..10 {
            let tx = tx.clone();
            s.spawn(move || {
                tx.send(i).unwrap();
            });
        }

        for _ in 0..5 {
            let rx1 = rx.clone();
            let rx2 = rx.clone();
            s.spawn(move || {
                let j = rx1.recv().unwrap();
                println!("线程#1: {}", j)
            });
            s.spawn(move || {
                let j = rx2.recv().unwrap();
                println!("线程#2: {}", j)
            });
        }
    })
}
