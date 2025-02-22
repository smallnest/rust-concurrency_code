use async_std::task;

#[async_std::main]
async fn main() {
    for i in 0..3 {
        task::spawn(async move{
            task::yield_now().await;
            println!("yielded in task {}", i);
        }).await;
    }

   
    task::yield_now().await;  
}