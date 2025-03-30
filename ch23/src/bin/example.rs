use std::sync::{Once, Arc};
use std::thread;

static mut RESOURCE: Option<i32> = None;
static INIT: Once = Once::new();

// 获取资源
fn get_resource() -> i32 {
    INIT.call_once(|| { // 只初始化一次
        unsafe {
            RESOURCE = Some(42); // 初始化资源
        }
        println!("资源初始化完成, by thread {:?}", thread::current().id());
    });

    unsafe { RESOURCE.unwrap() }
}

fn main() {
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn(move || {
            let value = get_resource();
            println!("线程 {:?}, 获取资源: {}", thread::current().id(), value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}