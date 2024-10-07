use std::thread;
use std::time::Duration;

fn main() {
        let handle1 = thread::spawn(|| {
            thread::sleep(Duration::from_millis(2000));
            println!("Hello from a thread1!");
        });
    
        let handle2 = thread::spawn(|| {
            thread::sleep(Duration::from_millis(1000));
            println!("Hello from a thread2!");
        });
    
        handle1.join().unwrap();
        handle2.join().unwrap();
    
}