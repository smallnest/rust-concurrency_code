use futures::{pending, select, FutureExt};

async fn pending_function() -> i32 {
    pending!(); 
    
    42
}

async fn ready_function() -> i32 {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    48
}


#[tokio::main]
async fn main() {
    select! {
        v = pending_function().fuse() => {
            println!("pending_function completed: {}",v);
        },
        v = ready_function().fuse() => {
            println!("ready_function completed: {}",v);
        }
    }
}

