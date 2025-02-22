async fn say_hello() {
    println!("Hello, world!");
}

#[async_std::main]
async fn main() {
    say_hello().await;
}


#[async_std::test]
async fn my_test() -> std::io::Result<()> {
    assert_eq!(2 * 2, 4);
    Ok(())
}

