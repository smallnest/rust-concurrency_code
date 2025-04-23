use sharded_slab::Slab;
use std::sync::Arc;

fn main() {
    let slab = Arc::new(Slab::new());

    let slab2 = slab.clone();
    let thread2 = std::thread::spawn(move || {
        let key = slab2.insert("来自线程2的问候").unwrap();
        assert_eq!(slab2.get(key).unwrap(), "来自线程2的问候");
        key
    });

    let key1 = slab.insert("来自线程1的问候").unwrap();
    assert_eq!(slab.get(key1).unwrap(), "来自线程1的问候");

    // 等待线程2完成
    let key2 = thread2.join().unwrap();

    // 查询线程2插入的值
    assert_eq!(slab.get(key2).unwrap(), "来自线程2的问候");
}
