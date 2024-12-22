use thread_priority::*;


fn main() {
    let thread = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn(|result| {
            // This is printed out from within the spawned thread.
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
    }).unwrap();
    thread.join().unwrap();
    
    
    let thread = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            println!("We don't care about the priority result.");
    }).unwrap();
    thread.join().unwrap();
}