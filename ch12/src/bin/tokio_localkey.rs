#[tokio::main]
async fn main() {
    tokio::task_local! {
        static NUMBER: u32;
    }
    
    NUMBER.scope(1, async move {
        println!("task local value: {}", NUMBER.get());
    }).await;

    NUMBER.scope(2, async move {
        println!("task local value: {}", NUMBER.get());
    }).await;

    NUMBER.sync_scope(3, || {
        println!("task local value: {}", NUMBER.get());
    });
}