use futures::future::{FutureExt, TryFutureExt, ready};
use tokio::runtime::Runtime;

async fn process_data(input: i32) -> Result<String, String> {
    // 使用 map 将 i32 转换为 String
    let string_future = ready(Ok(input)).map(|result| result.map(|n| format!("Number: {}", n)));

    // 使用 and_then 处理 Result，并在成功时返回另一个 Future
    string_future
        .and_then(|s| async move {
            Ok(format!("Processed: {}", s))
        })
        .await
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result1 = process_data(10).await;
        println!("Result 1: {:?}", result1); // 输出：Ok("Processed: Number: 10")

        let result2 = process_data(-1).await; // 假设 -1 是一个错误输入
        println!("Result 2: {:?}", result2); // 输出：Ok("Processed: Number: -1")
    });
}
