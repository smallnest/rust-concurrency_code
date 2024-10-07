use std::thread;

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::spawn(move || {
        println!("hello from the first scoped thread");
        dbg!(&a);
    });
    thread::spawn(move || {
        println!("hello from the second scoped thread");
        x += a[0] + a[2];
    });
    println!("hello from the main thread");

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}