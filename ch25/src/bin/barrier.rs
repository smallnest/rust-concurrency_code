use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
	let barrier = Arc::new(Barrier::new(3));
	let mut handles = vec![];
	for i in 0..3 {
		let b = barrier.clone();
		handles.push(thread::spawn(move || {
			println!("Thread {} is waiting at the barrier", i);
			let result = b.wait();
			println!("Thread {} has passed the barrier. is leader: {}", i, result.is_leader());
		}
		));
	}
	for handle in handles {
		handle.join().unwrap();
	}
	println!("All threads have passed the barrier");
}