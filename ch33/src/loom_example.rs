use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

#[test]
fn test_concurrent_logic() {
    let v1 = Arc::new(AtomicUsize::new(0));
    let v2 = v1.clone();

    thread::spawn(move || {
        v1.store(1, SeqCst);
    });

    assert_eq!(0, v2.load(SeqCst));
}

#[test]
fn test_concurrent_logic_with_loom() {
    loom::model(|| {
        let v1 = Arc::new(AtomicUsize::new(0));
        let v2 = v1.clone();

        thread::spawn(move || {
            v1.store(1, SeqCst);
        });

        assert_eq!(0, v2.load(SeqCst));
    });
}