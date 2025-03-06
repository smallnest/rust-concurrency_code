use std::sync::Mutex;
fn main() {
    let mutex = Mutex::new(0);
    let mutex_inner = mutex.into_inner().unwrap();
    assert_eq!(mutex_inner, 0);

    // let _ = mutex.lock().unwrap();
}