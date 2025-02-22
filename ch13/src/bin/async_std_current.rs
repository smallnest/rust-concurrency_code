use async_std::task;

#[async_std::main]
async fn main() {
    task::spawn(async {
        println!("current task: {}", task::current().id());
    }).await;
    
    task::spawn_blocking(|| {
        let _ = task::Builder::new().name("child1".to_string()).spawn(async {
            println!("current task: {:?}", task::try_current());
        });
    })
    .await;
}