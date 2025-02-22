use std::time::Duration;
use async_std::task;

#[async_std::main]
async fn main() {
    task::sleep(Duration::from_secs(1)).await;

    // std::thread::sleep(Duration::from_secs(1));
}
