use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let foo = AtomicBool::new(true);
    assert_eq!(foo.fetch_and(false, Ordering::SeqCst), true);
    assert_eq!(foo.load(Ordering::SeqCst), false);

    let foo = AtomicBool::new(true);
    assert_eq!(foo.fetch_and(true, Ordering::SeqCst), true);
    assert_eq!(foo.load(Ordering::SeqCst), true);

    let foo = AtomicBool::new(false);
    assert_eq!(foo.fetch_and(false, Ordering::SeqCst), false);
    assert_eq!(foo.load(Ordering::SeqCst), false);
}
