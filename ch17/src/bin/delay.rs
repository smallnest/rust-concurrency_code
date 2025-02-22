use std::future::Future;
use std::task::{Context, Poll};
use std::time::Duration;

struct Delay {
    duration: Duration,
    completed: bool,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Delay {
            duration,
            completed: false,
        }
    }
}

impl Future for Delay {
    type Output = String;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("延迟完成！".to_string())
        } else {
            println!("子曰：三思而后行！");

            // 模拟耗时操作，这里使用线程休眠
            std::thread::sleep(self.duration);
            self.completed = true;
            cx.waker().wake_by_ref(); // 非常重要！唤醒执行器
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let delay = Delay::new(Duration::from_secs(2));
    let result = delay.await;
    println!("{}", result);
}
