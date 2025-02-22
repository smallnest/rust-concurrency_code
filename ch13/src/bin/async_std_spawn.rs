use async_std::task;

#[async_std::main]
async fn main() {
    let v = task::spawn(async {
        1 + 2
    }).await;
    assert_eq!(v, 3);
    
    task::spawn_blocking(|| {
        println!("long-running task here");
    })
    .await;

    let v = task::spawn_local(async {
        1 + 2
    }).await;
    assert_eq!(v, 3);
}