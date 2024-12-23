use std::thread;
use std::time::Duration;
use parking::Parker;

fn main() {
    let p = Parker::new();
    let u = p.unparker();

    // 通知 parker
    u.unpark();

    // 立刻被唤醒，因为 parker 已经被通知
    p.park();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        u.unpark();
    });

    // 等待被唤醒
    p.park();

    println!("park_unpark");


    let (p,u) = parking::pair();
    // 重复调用 unpark 也是安全的
    u.clone().unpark();
    u.clone().unpark();

    p.park();
    println!("park_unpark");
    
    p.park();
    println!("park_unpark again");

}