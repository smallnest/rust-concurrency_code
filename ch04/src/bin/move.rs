use std::thread;

fn main() {
    let x = 100;

    let handle = thread::spawn(move || { // ①
        println!("Hello from a thread with move, x={}!", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(|| { // ②
        println!("Hello from a thread without move");
    });
    handle.join().unwrap();
}
