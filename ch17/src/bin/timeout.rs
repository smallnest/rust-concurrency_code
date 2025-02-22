use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;

#[pin_project]
pub struct Timeout<F, D> {
    #[pin]
    future: F,
    #[pin]
    deadline: D,
    completed: bool,
}

impl<F, D> Timeout<F, D> {
    pub fn new(future: F, deadline: D) -> Self {
        Self {
            future,
            deadline,
            completed: false,
        }
    }
}

impl<F: Future, D: Future> Future for Timeout<F, D> {
    type Output = io::Result<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        assert!(!*this.completed, "future polled after completing");

        match this.future.poll(cx) {
            Poll::Ready(v) => {
                *this.completed = true;
                Poll::Ready(Ok(v))
            }
            Poll::Pending => match this.deadline.poll(cx) {
                Poll::Ready(_) => {
                    *this.completed = true;
                    Poll::Ready(Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out")))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

#[tokio::main]
async fn main() {
    // 创建一个延迟 2 秒的 Future
    let delay = tokio::time::sleep(std::time::Duration::from_secs(2));
    // 创建一个超时时间为 1 秒的 Future
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(1));

    // 创建一个超时 Future
    let timeout_future = Timeout::new(delay, timeout);

    match timeout_future.await {
        Ok(_) => println!("Future 完成！"),
        Err(e) => eprintln!("Future 超时！错误：{}", e),
    }
}
