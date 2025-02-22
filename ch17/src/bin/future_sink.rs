use futures::channel::mpsc;
use futures::prelude::*;
use tokio::runtime::Runtime;

async fn send_data(mut tx: mpsc::Sender<i32>) {
    for i in 1..=5 {
        tx.send(i).await.unwrap();
        println!("Sent: {}", i);
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

async fn receive_data(mut rx: mpsc::Receiver<i32>) {
    while let Some(item) = rx.next().await {
        println!("Received: {}", item);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, rx) = mpsc::channel(10); // 创建一个容量为 10 的通道

        tokio::join!(send_data(tx), receive_data(rx));
    });
}