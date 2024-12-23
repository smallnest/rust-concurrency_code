use std::thread;
use thread_control::*;

fn main() {
    let (flag, control) = make_pair();
    let handle = thread::spawn(move || {
        while flag.alive() {
        }
    });
    assert_eq!(control.is_done(), false);
    control.stop();
    let _ = handle.join();
    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);
}