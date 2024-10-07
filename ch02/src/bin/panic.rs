use std::thread;

fn main() {
    // 创建一个线程
    let handle = thread::spawn(|| {
        let a = 1;
        let b = 0;
        print!("{} / {} = {}", a, b, a / b);
    });

    // 等待子线程完成，并捕获返回值
    let result = handle.join();
    match result {
        Ok(_) => println!("子线程正常结束"),
        Err(e) => {
            
            if let Some(s) = e.downcast_ref::<String>() {
                println!("主线程捕获到子线程的恐慌: {}", s);
            } else if let Some(s) = e.downcast_ref::<&'static str>() {
                println!("主线程捕获到子线程的恐慌: {}", s);
            } else {
                println!("主线程捕获到子线程的恐慌: {:?}", e);
            }
        }
    }

    println!("主线程继续运行...");
}



