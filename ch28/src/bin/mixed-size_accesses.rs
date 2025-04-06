use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let x = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    unsafe { 
        let  y = x.as_ptr() as *const AtomicU32;
        let y1 =Arc::new(y.as_ref().unwrap());
    
        let handle1 = thread::spawn(move || {
            // 32位原子读取
            let v = y1.load(Ordering::SeqCst);
            println!("32位原子读取的值: {}", v);
        });

        handles.push(handle1);
    }

    let x2 = Arc::clone(&x);
    let handle2 = thread::spawn(move || {
        // 64位原子写入
        x2.store(42, Ordering::SeqCst);
        println!("64位原子写入的值: {}", x2.load(Ordering::SeqCst));
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }
}
