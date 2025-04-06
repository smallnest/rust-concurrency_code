use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    // 创建一个信号量，初始许可数为5
    let semaphore = Semaphore::new(5);

    // 请求3个许可
    let permit = semaphore.acquire_many(3).await.unwrap();
    assert_eq!(semaphore.available_permits(), 2); // 剩余许可数为2
}