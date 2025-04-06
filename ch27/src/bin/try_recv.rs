use std::sync::mpsc::{Receiver, channel};

fn main() {
    let (_, receiver): (_, Receiver<i32>) = channel();

    assert!(receiver.try_recv().is_err());
}