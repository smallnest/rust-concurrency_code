fn main() {
    let q = lockfree::queue::Queue::new();

    // 启动一个线程往队列中写
    std::thread::scope(|s| for i in 0..6 {
        let q = &q;

        s.spawn(move || {
            // 压入一个元素到队列中
            q.push(i);
        });

        s.spawn(move || {
            match  q.pop() {
                Some(x) => println!("弹出: {}", x),
                None => println!("队列为空"),
                
            }
        });
    });

    for i in q {
        println!("drain item: {}", i);
    }
}