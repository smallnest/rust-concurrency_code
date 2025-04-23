fn main() {
    let vec = boxcar::Vec::new();

    // 启动一个线程往vec中写
    std::thread::scope(|s| for i in 0..6 {
        let vec = &vec;
    
        s.spawn(move || {
            // 插入一个元素到vec中
            vec.push(i);
        });

        s.spawn(move || {
            // 打印vec的长度
            println!("vec len: {}", vec.count());
        });
    });

    // 在主线程中读取vec中的元素
    for i in 0..6 {
        assert!(vec.iter().any(|(_, &x)| x == i));
    }
}
