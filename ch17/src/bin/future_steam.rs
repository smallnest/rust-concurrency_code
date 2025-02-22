use futures::stream::{self, StreamExt};
use tokio::runtime::Runtime;

async fn process_stream() {
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);

    stream.for_each(|x| async move {
        println!("Processing: {}", x);
        // 这里可以进行一些异步操作，例如网络请求
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }).await;

    println!("Stream processing complete.");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        process_stream().await;
    });
}