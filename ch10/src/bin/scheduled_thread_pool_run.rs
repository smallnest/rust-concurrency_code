use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();

    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let handle = pool.execute_after(Duration::from_millis(1000), move || {
        println!("Hello from a scheduled thread!");
        sender.send("done").unwrap();
    });

    let _ = handle;
    receiver.recv().unwrap();

    let handle = pool.execute_at_fixed_rate(
        Duration::from_millis(1000),
        Duration::from_millis(1000),
        || {
            println!("Hello from a scheduled thread!");
        },
    );

    sleep(Duration::from_secs(5));
    handle.cancel()
}
