use async_std::task;

async fn process_item(item: i32) -> i32 {
    // 非常简单的操作
    item * 2
}

async fn bad_use_of_spawn() {
    let mut results = Vec::new();
    for i in 0..10000 {
        // 错误：为每个简单的操作都 spawn 一个任务
        let handle = task::spawn(process_item(i));
        results.push(handle.await);
    }
    println!("{:?}",results.len());
}

async fn good_use_of_spawn() {
    let mut results = Vec::new();
    for i in 0..10000{
      results.push(process_item(i).await);
    }
    println!("{:?}",results.len());
}

fn main() {
    task::block_on(async {
        bad_use_of_spawn().await;
        good_use_of_spawn().await;
    });
}