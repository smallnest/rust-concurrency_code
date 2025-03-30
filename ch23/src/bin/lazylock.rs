use std::sync::LazyLock;
use std::thread;

static RESOURCE: LazyLock<i32> = LazyLock::new(|| {
    println!("资源初始化完成, by thread {:?}", thread::current().id());
    42
});


fn main() {
    let mut handles = vec![];

    for _ in 0..5 {
        let handle = thread::spawn(move || {
            let value = *RESOURCE;
            println!("线程 {:?}, 获取资源: {}", thread::current().id(), value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}