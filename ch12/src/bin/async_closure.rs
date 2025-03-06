use futures::join;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    {
        let async_closure = async || {
            println!("Hello from async closure!");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        };

        async_closure().await;
    }

    takes_an_async_f_by_tokio(async |s| {
        other_fn(s).await;
    })
    .await;
}

async fn other_fn(s: &str) {
    println!("Received: {}", s);
}

async fn takes_an_async_f_by_futures(f: impl AsyncFn(&str)) {
    // join!(f("hello"), f("world")).await;
}

async fn takes_an_async_f_by_tokio(f: impl AsyncFn(&str)) {
    tokio::join!(f("hello"), f("world"));
}


async fn takes_an_async_fn(f: impl AsyncFn(&str)) {
    f("hello").await;
    f("world").await;
}
    