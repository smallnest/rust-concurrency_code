use std::thread::current;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .thread_name(|i| format!("rayon-global-thread-{}", i))
        .build_global()?;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!(
                "Hello from rayon global thread:{}!",
                current().name().unwrap()
            );
        });
    });



    Ok(())
}
