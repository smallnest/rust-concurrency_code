use futures::join;
use tokio::runtime::Runtime;

async fn fetch_data(url: &str) -> Result<String, String> {
    // 模拟网络请求
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(format!("Data from {}", url))
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let future1 = fetch_data("url1");
        let future2 = fetch_data("url2");

        let (result1, result2) = join!(future1, future2);

        println!("Result 1: {:?}", result1);
        println!("Result 2: {:?}", result2);
    });
}