use std::sync::mpsc;
use std::thread;

fn main() {
    let num_workers = 4;
    let (tx, rx) = mpsc::channel();

    for i in 0..num_workers {
        let tx_clone = tx.clone();
        thread::spawn(move |
| {
            // 模拟一些计算
            let result = i * i;
            println!("Worker {} finished, sending result: {}", i, result);
            tx_clone.send(result).unwrap();
        });
    }

    let mut results = Vec::new();
    for _ in 0..num_workers {
        results.push(rx.recv().unwrap());
    }

    println!("All results received: {:?}", results);
}