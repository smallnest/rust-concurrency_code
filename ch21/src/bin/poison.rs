use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let data_cloned = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut data = data_cloned.lock().unwrap();
        data.push(4);
        panic!("oops!");  // 持锁线程崩溃，Mutex变poisoned
    });

    let _ = handle.join();  // 这里会看到线程panic，但Mutex还存在

    // 其他线程尝试加锁
    match data.lock() {
        Ok(guard) => {
            println!("Got lock successfully: {:?}", *guard);
        }
        Err(poisoned) => {
            println!("Lock is poisoned! Recovering...");
            let guard = poisoned.into_inner();  // 直接获取guard，放弃Poison检测
            println!("Recovered data: {:?}", *guard);
        }
    }
}
// 这里我们创建了一个Arc<Mutex<Vec<i32>>>，并在一个线程中对其进行修改，然后panic。
// 在主线程中，我们尝试再次加锁，这时Mutex已经被poisoned，我们可以通过into_inner()方法获取Mutex内部的数据。
// 这里我们放弃了Poison检测，直接获取了Mutex内部的数据，这样就可以继续使用这个数据了。
// 这种情况下，我们需要自己处理poisoned的情况，否则程序会直接panic。

// 你说我们放弃了Poison检测，怎么进行Poison检测?
// 通过调用Mutex的is_poisoned()方法，我们可以检测Mutex是否被poisoned。
// 通过调用Mutex的get_mut()方法，我们可以获取Mutex内部的数据的可变引用，这样我们就可以继续使用这个数据了。