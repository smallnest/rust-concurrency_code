use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep, Sleep};
use pin_project::pin_project;

#[pin_project] // 使用 pin_project 宏
struct Delay {
    #[pin] // 标记需要 Pin 住的字段
    sleep: Sleep,
    message: String,
}

impl Delay {
    fn new(duration: Duration, message: String) -> Self {
        Delay {
            sleep: sleep(duration),
            message,
        }
    }
}

impl Future for Delay {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project(); // 使用 project() 方法进行投影

        match this.sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(this.message.clone()),
        }
    }
}

#[tokio::main]
async fn main() {
    let delay1 = Delay::new(Duration::from_secs(2), "第一个延迟完成！".to_string());
    let delay2 = Delay::new(Duration::from_secs(1), "第二个延迟完成！".to_string());

    tokio::join!(async { // 并发执行两个 Future
        let result1 = delay1.await;
        println!("{}", result1);
    }, async {
        let result2 = delay2.await;
        println!("{}", result2);
    });

    println!("所有延迟完成！");
}