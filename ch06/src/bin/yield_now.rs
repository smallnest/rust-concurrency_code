use std::thread;

fn main() {
        let handle1 = thread::spawn(|| {
            thread::yield_now(); // ①
            println!("yield_now!");
        });
    
        let handle2 = thread::spawn(|| {
            thread::yield_now(); // ②
            println!("yield_now in another thread!");
        });
    
        handle1.join().unwrap();
        handle2.join().unwrap();
}