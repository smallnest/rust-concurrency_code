use std::thread;

fn main() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let handle1 = thread::spawn(move|| {
        println!("hello from the first thread");
        dbg!(&x);
    });

    let handle2 = thread::spawn(move|| {
        println!("hello from the second thread");
        let v = x[0] + x[2];
        println!("v = {}", v);
    });

    println!("hello from the main thread");

    handle1.join().unwrap();
    handle2.join().unwrap();
}