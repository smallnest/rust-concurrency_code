use async_std::task::Builder;

#[async_std::main]
async fn main() {
    let _ = Builder::new().name("child1".to_string()).spawn(async {
        println!("Hello, world!");
    });
}