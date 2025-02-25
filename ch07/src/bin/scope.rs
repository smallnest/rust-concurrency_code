use std::thread;

fn main() {
    let result = thread::scope(|s| {
        s.spawn(|| {
            println!("Thread 1 is running");
        });

        s.spawn(|| {
            println!("Thread 2 is running");
        });

        // 返回一个值
        42
    });


    println!("Scope completed successfully with value: {}", result)
}