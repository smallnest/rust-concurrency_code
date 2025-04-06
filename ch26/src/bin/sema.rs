use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Semaphore::new(2);

    // 请求一个许可
    let permit_1 = semaphore.acquire().await.unwrap();
    assert_eq!(semaphore.available_permits(), 1);

    // 请求另一个许可
    let permit_2 = semaphore.acquire().await.unwrap();
    assert_eq!(semaphore.available_permits(), 0);

    // 主动释放第一个许可
    drop(permit_1);
    assert_eq!(semaphore.available_permits(), 1);
}