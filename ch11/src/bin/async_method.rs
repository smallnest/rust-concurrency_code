use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn my_async_method(&self, x: i32) -> i32;
}

struct MyStruct;

#[async_trait]
impl MyTrait for MyStruct {
    async fn my_async_method(&self, x: i32) -> i32 {
        println!("在 MyStruct 中执行异步方法，输入值为：{}", x);
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        println!("MyStruct 中的异步方法完成");
        x * 3
    }
}

#[tokio::main]
async fn main() {
    let my_struct = MyStruct;
    let result = my_struct.my_async_method(10).await;
    println!("异步方法的结果是：{}", result);
}