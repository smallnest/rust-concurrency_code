use std::thread;
use thread_priority::*;
use std::convert::*;

fn main() {
    thread_priority_crossplatform();
    thread_priority_min_max();
}

#[allow(dead_code)]
fn thread_priority_crossplatform() {
    let handle1 = thread::spawn(|| {
        let v =  ThreadPriorityValue::try_from(15u8).unwrap();
        
        match set_current_thread_priority(ThreadPriority::Crossplatform(v)) {
            Ok(_) => println!("Hello from a thread5!"),
            Err(e) => println!("Thread5 priority set failed: {:?}", e),
        }

        ;
    });

    let handle2 = thread::spawn(|| {
        let v =  ThreadPriorityValue::try_from(20u8).unwrap();
        match set_current_thread_priority(ThreadPriority::Crossplatform(v)) {
            Ok(_) => println!("Hello from a thread6!"),
            Err(e) => println!("Thread6 priority set failed: {:?}", e),
            
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[allow(dead_code)]
fn thread_priority_min_max() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });

    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}