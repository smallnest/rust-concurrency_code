use fast_threadpool::{ThreadPool, ThreadPoolConfig};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threadpool = ThreadPool::start(ThreadPoolConfig::default(), ()).into_async_handler();

    let _ = threadpool.execute(|_| {
        println!("Hello from my custom thread!");
    }).await?;

    Ok(())
}