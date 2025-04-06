use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{interval, Duration};

struct TokenBucket {
    sem: Arc<Semaphore>,
    jh: tokio::task::JoinHandle<()>,
}

impl TokenBucket {
    fn new(duration: Duration, capacity: usize) -> Self {
        let sem = Arc::new(Semaphore::new(capacity));

        // 每个周期内，信号量的许可数会增加1。
        let jh = tokio::spawn({
            let sem = sem.clone();
            let mut interval = interval(duration);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            async move {
                loop {
                    interval.tick().await;

                    if sem.available_permits() < capacity {
                        sem.add_permits(1);
                    }
                }
            }
        });

        Self { jh, sem }
    }

    async fn acquire(&self) {
        // 请求1个许可，然后信号量会减少一个
        let permit = self.sem.acquire().await.unwrap();
        permit.forget();
    }
}

impl Drop for TokenBucket {
    fn drop(&mut self) {
        // 这是一个小技巧，在drop时取消任务，避免泄露。
        self.jh.abort();
    }
}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let capacity = 3;
    let update_interval = Duration::from_secs_f32(1.0 / capacity as f32);
    let bucket = TokenBucket::new(update_interval, capacity);

    // 应该每秒释放三个许可
    loop  {
        bucket.acquire().await;  
        log::info!("acquired a token");
    }
}