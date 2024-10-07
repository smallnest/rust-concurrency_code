// use std::thread;

// fn main() {
//     let mut a = vec![1, 2, 3];
//     let mut x = 0;

//     thread::scope(|s| {
//         s.spawn(|| {
//             println!("hello from the first scoped thread");
//             x += a[0] + a[2] + 10;
//         });
//         s.spawn(|| {
//             println!("hello from the second scoped thread");
//             x += a[0] + a[2];
//         });
//         println!("hello from the main thread");
//     });

//     a.push(4);
//     assert_eq!(x, a.len());
// }