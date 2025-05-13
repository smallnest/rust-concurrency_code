#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_mpsc() {
        let (tx, rx) = mpsc::channel::<u32>();

        let producer = thread::spawn(move || {
            // tx.send(42).unwrap();
        });

        // 没有生产者发送数据，消费者持久等待
        thread::sleep(std::time::Duration::from_millis(100));
        match rx.recv() {
            Ok(value) => println!("Received: {}", value),
            Err(_) => println!("Channel closed unexpectedly"),
        }

        producer.join().unwrap()
    }
}
