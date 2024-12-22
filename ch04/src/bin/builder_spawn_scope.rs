use std::thread;

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        //① 我们可以在这里创建线程，并在作用域内使用 `a` 和 `x`。
        thread::Builder::new()
            .name("first_thread".to_string())
            .spawn_scoped(s, || {
                println!(
                    "hello from the {:?} scoped thread",
                    thread::current().name()
                );
                // 我们可以在这里借用 `a`。
                dbg!(&a);
            })
            .unwrap();
        //② 我们可以在这里创建另一个线程，并在作用域内使用 `a` 和 `x`。
        thread::Builder::new()
            .name("second_thread".to_string())
            .spawn_scoped(s, || {
                println!(
                    "hello from the {:?} scoped thread",
                    thread::current().name()
                );
                // 我们甚至可以在这里可变地借用 `x`，
                // 因为没有其他线程在使用它。
                x += a[0] + a[2];
            })
            .unwrap();
        println!("hello from the main thread");
    });

    // 在作用域之后，我们可以再次修改和访问我们的变量：
    a.push(4);
    assert_eq!(x, a.len());
    dbg!(&a);
}
