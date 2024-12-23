use std::thread;

macro_rules! join_all {
    ($handles:expr) => {
        for handle in $handles {
            handle.join().unwrap();
        }
    };
}

fn main() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    join_all!([handle1, handle2]); // 使用数组字面量

    let mut handles = Vec::new();
    handles.push(thread::spawn(|| println!("Hello from thread 3")));
    handles.push(thread::spawn(|| println!("Hello from thread 4")));

    join_all!(handles); // 使用 Vec

    let handles2 = vec![
        thread::spawn(|| println!("Hello from thread 5")),
        thread::spawn(|| println!("Hello from thread 6")),
    ];
    join_all!(handles2); // 使用 vec![] 宏

    let handles3 = (0..2).map(|i| thread::spawn(move|| println!("Hello from thread {}", i))).collect::<Vec<_>>();
    join_all!(handles3); // 使用迭代器和 collect

}