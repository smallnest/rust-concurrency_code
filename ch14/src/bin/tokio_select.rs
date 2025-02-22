use tokio::select;
use tokio::time::{sleep, Duration};

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Resource dropped");
    }
}

async fn operation_with_resource() {
    let _resource = Resource;
    println!("Operation with resource started");
    sleep(Duration::from_secs(2)).await;
    println!("Operation with resource finished");
}

#[tokio::main]
async fn main() {
    select! {
        _ = operation_with_resource() => {
            println!("Operation branch completed");
        }
        _ = sleep(Duration::from_secs(1)) => {
            println!("Timeout branch completed"); // 如果这个分支先完成
        }
    }
    println!("Main function finished");
}