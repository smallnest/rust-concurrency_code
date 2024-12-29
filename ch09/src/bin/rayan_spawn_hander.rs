use std::time;

fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .spawn_handler(|thread| {
            std::thread::spawn(|| {
                println!("start a new thread");
                thread.run()
            });
            Ok(())
        })
        .start_handler(|i| {
            println!("start thread #{}", i);
        })
        .exit_handler(|i| {
            println!("exit thread #{}", i);
        })
        .build()?;

    pool.install(|| println!("Hello from my custom thread!"));
    
    drop(pool);
    std::thread::sleep(time::Duration::from_secs(1));
    
    Ok(())
}
