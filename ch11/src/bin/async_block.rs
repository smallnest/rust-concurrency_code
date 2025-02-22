#[tokio::main]
async fn main() {
    let x = 5;
    let my_async_closure = async move |y: i32| -> i32 {
        println!("在闭包中执行异步操作，输入值为：x={}, y={}", x, y); // 注意这里捕获了外部变量 x
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        println!("闭包中的异步操作完成");
        x + y
    };

    let result = my_async_closure(10).await;
    println!("异步闭包的结果是：{}", result);
}