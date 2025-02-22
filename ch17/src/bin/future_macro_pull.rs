use std::task::Poll;
use futures::*;

async fn some_async_function() -> Result<&'static str, ()> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Async function completed")
}

#[tokio::main]
async fn main() {
    let mut future = Box::pin(some_async_function());
    let poll_result = poll!(&mut future); // 这一行轮询 future

    // 检查轮询结果
    match poll_result {
        Poll::Ready(value) => {
            // future 成功完成，处理该值
            println!("Future resolved with: {:?}", value);
        }
        Poll::Pending => {
            // future 尚未准备好，处理这种情况（例如，yield）
            println!("Future is not ready yet");
        }
    }

    
    
    // 等待足够的时间，重新轮询 future
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let poll_result = poll!(&mut future);
    match poll_result {
        Poll::Ready(value) => {
            println!("Future resolved with: {:?}", value);
        }
        Poll::Pending => {
            println!("Future is not ready yet");
        }
    }
}
