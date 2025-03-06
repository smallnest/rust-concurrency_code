use futures::join;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // 1. 普通闭包
    let closure = || {
        println!("This is a sync closure");
    };
    closure();

    // 2. 普通闭包，利用异步块返回 Future
    let closure = || async{
        println!("This is a sync closure with async block");
    };
    closure().await;

    // 3. 异步闭包
    let async_closure = async || {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("This is an async closure");
    };
    async_closure().await;

    sync_closure_call(closure).await;
    async_closure_call(async_closure).await;
}

async fn async_closure_call(f: impl AsyncFn()) {
    f().await;
}

async fn sync_closure_call<F, Fut>(f: F) 
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    f().await;
}