use std::sync::atomic::{AtomicI16, Ordering};

fn main() {
    let mut some_var = AtomicI16::new(10);
    assert_eq!(*some_var.get_mut(), 10);
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);
}