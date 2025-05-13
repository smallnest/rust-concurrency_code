#[cfg(test)]
mod tests {
    use std::sync::{Arc,Condvar, Mutex};
    use std::thread;

    #[test]
    fn test_condvar() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_clone = Arc::clone(&pair);

        let producer = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(50));

            let (lock, cvar) = &*pair_clone;
            let mut ready = lock.lock().unwrap();
            *ready = true;
            cvar.notify_one(); // 通知消费者
        });

        let consumer = thread::spawn(move || {
            let (lock, cvar) = &*pair;
            let ready = lock.lock().unwrap();

            // 如果消费者在生产者通知之前进入等待，可能错过通知；或者虚假唤醒导致条件未满足。
            let ready = cvar.wait(ready).unwrap();
            println!("Ready: {}", *ready);
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }
}
