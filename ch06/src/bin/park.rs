use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        thread::park();
        println!("Hello from a park thread!");
    });

    thread::sleep(Duration::from_millis(1000));

    handle.thread().unpark();

    handle.join().unwrap();

    unpark_first();

    park_multile_times();
}

fn unpark_first() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

    handle.thread().unpark();

    handle.join().unwrap();
}


fn park_multile_times() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        thread::park();
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });
    handle.thread().unpark();
    handle.thread().unpark();
    handle.thread().unpark();
    handle.join().unwrap();
}
