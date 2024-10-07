use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("当前线程的名称: {:?}", current_thread.name().unwrap());
    println!("当前线程的 ID: {:?}", current_thread.id());
}