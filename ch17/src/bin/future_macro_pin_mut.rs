use futures::pin_mut;
use std::pin::Pin;

fn main() {
    let value = 5;
    pin_mut!(value); // value 现在是 Pin<&mut i32>


    let mut pinned_value: Pin<&mut i32> = value;
    *pinned_value = 10;
    
    println!("{}", *pinned_value); // 输出 10
}