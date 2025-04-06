use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    // 创建两个原子布尔标志
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    
    // 用于存储观察结果的变量
    let mut saw_x_not_y = false;
    let mut saw_y_not_x = false;
    
    // 克隆用于线程的引用
    let x_clone1 = Arc::clone(&x);
    let y_clone1 = Arc::clone(&y);
    let x_clone2 = Arc::clone(&x);
    let y_clone2 = Arc::clone(&y);
    
    // 启动第一个线程
    let thread1 = thread::spawn(move || {
        // 设置x为true
        x_clone1.store(true, Ordering::SeqCst);
        
        // 检查y是否为true
        // 使用SeqCst保证全局一致的操作顺序
        if !y_clone1.load(Ordering::SeqCst) {
            saw_x_not_y = true;
            println!("线程1看到: x=true, y=false");
        } else {
            println!("线程1看到: x=true, y=true");
        }
    });
    
    // 启动第二个线程
    let thread2 = thread::spawn(move || {
        // 设置y为true
        y_clone2.store(true, Ordering::SeqCst);
        
        // 检查x是否为true
        // 使用SeqCst保证全局一致的操作顺序
        if !x_clone2.load(Ordering::SeqCst) {
            saw_y_not_x = true;
            println!("线程2看到: x=false, y=true");
        } else {
            println!("线程2看到: x=true, y=true");
        }
    });
    
    // 等待线程完成
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    // 重要的SeqCst保证：不可能两个线程都看到对方的标志为false
    // 也就是说，saw_x_not_y和saw_y_not_x不可能同时为true
    println!("\n结果分析:");
    println!("线程1看到x=true但y=false: {}", saw_x_not_y);
    println!("线程2看到y=true但x=false: {}", saw_y_not_x);
    
    if saw_x_not_y && saw_y_not_x {
        println!("错误! 这在使用SeqCst时不应该发生");
    } else {
        println!("符合SeqCst保证: 两个线程不可能都看不到对方的写入");
    }
}