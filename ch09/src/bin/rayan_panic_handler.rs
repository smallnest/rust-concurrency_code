fn main() -> Result<(), rayon::ThreadPoolBuildError> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .panic_handler(|panic_info| {
            let thread_id = std::thread::current().id();
            let panic_info = format!("{:?}", panic_info);

            println!("=== Panic 信息 ===");
            println!("线程: {:?}", thread_id);
            println!("错误: {}", panic_info);
        })
        .build()?;

    pool.spawn(|| panic!("Panic from my custom thread!"));


    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())

}