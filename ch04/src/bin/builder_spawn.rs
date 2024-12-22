use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker_thread".to_string());

    let handle = builder.spawn(|| {
        println!("这是一个工作线程: {}", thread::current().name().unwrap());
    }).unwrap();

    handle.join().unwrap();
}

