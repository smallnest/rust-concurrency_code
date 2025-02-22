use async_std::future;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let never = future::pending::<()>();
    let dur = Duration::from_millis(5);
    assert!(future::timeout(dur, never).await.is_err());

    let val = future::ready(5);
    let dur = Duration::from_millis(5);
    assert!(future::timeout(dur, val).await.is_ok());
}
