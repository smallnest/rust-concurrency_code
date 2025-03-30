use std::thread;

fn main() {
    let builder = thread::Builder::new();

    let mut x = 1;
    let thread_x = &x;

    let handler = unsafe {
        builder
            .spawn_unchecked(move || {
                println!("x = {}", *thread_x);
            })
            .unwrap()
    };

    handler.join().unwrap();
}


