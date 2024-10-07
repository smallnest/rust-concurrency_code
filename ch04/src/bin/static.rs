use std::thread;

static X: [i32; 3] = [1, 2, 3];

fn main() {
    let handle1 = thread::spawn(|| {
        println!("hello from the first thread");
        dbg!(&X);
    });

    let handle2 = thread::spawn(|| {
        println!("hello from the second thread");
        let x = X[0] + X[2];
        println!("x = {}", x);
    });

    println!("hello from the main thread");

    handle1.join().unwrap();
    handle2.join().unwrap();
}