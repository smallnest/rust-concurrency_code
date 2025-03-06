use std::sync::RwLock;

fn main() {
    // deadlock1();
    // deadlock2();
    // deadlock3();
    // deadlock4();
}

// 严格说来不是死锁，而是hang住了
fn deadlock1() { 
    let lock = RwLock::new(5);

    // 同线程中，先获取读锁，再获取写锁
    let r1 = lock.read().unwrap();
    let mut w = lock.write().unwrap(); // 阻塞
    *w += 1;
    println!("r1 = {}", *r1);
}


fn deadlock2() {
    let lock = RwLock::new(5);

    // 同线程中，先获取读锁，再获取写锁
    let mut w1 = lock.write().unwrap(); // 阻塞
    let mut w2 = lock.write().unwrap(); // 阻塞
    *w2 += 1;
    println!("r1 = {}", *lock.read().unwrap());
}


fn deadlock3() { 
    let lock = std::sync::Arc::new(RwLock::new(5));
    
    let lock1 = lock.clone();
    let lock2 = lock.clone();
    let handle1 = std::thread::spawn(move || {
        let r1 = lock1.read().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let mut w = lock1.write().unwrap();
        *w += 1;

        println!("r1 = {}", *r1);
    });

    let handle2 = std::thread::spawn(move || {
        let mut w = lock2.write().unwrap();
        *w += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn deadlock4() { 
    let lock = std::sync::Arc::new(RwLock::new(5));
    
    let lock1 = lock.clone();
    let lock2 = lock.clone();
    let handle1 = std::thread::spawn(move || {
        let r1 = lock1.read().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        let r2 = lock1.read().unwrap(); // 可能阻塞，也可能不阻塞

        println!("r1 = {}, r2 = {}", *r1, *r2);
    });

    let handle2 = std::thread::spawn(move || {
        let mut w = lock2.write().unwrap();
        *w += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
