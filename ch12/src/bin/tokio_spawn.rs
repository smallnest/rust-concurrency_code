use std::rc::Rc;
use tokio::task;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    {
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

    {
        let result = task::spawn_blocking(move || {
            thread::sleep(Duration::from_secs(2));
            return 200
        })
        .await
        .unwrap();

        println!("Result: {}", result); // 输出 20
    }

    {
        let nonsend_data = Rc::new("my nonsend data...");
        // 创建了一个新的 LocalSet，它是一个任务集合，
        // 保证这些任务在同一个线程上运行。
        // 这对于需要访问非线程安全数据的任务非常有用。
        let local = task::LocalSet::new();

        // 使用 run_until 方法运行本地任务集，
        // 直到提供的异步任务完成。在这个异步任务中，
        // 首先克隆了 nonsend_data，以确保每个任务都有自己的数据引用。
        local
            .run_until(async move {
                let nonsend_data = nonsend_data.clone();
                // 使用 task::spawn_local 生成一个新的本地任务。
                // 这个函数类似于 tokio::spawn，
                // 但它确保任务在与 LocalSet 相同的线程上运行。
                // 在这个任务中，打印了 nonsend_data 的内容。
                // await 和 unwrap 确保等待任务完成
                task::spawn_local(async move {
                    println!("{}", nonsend_data);
                    // ...
                })
                .await
                .unwrap();
            })
            .await;
    }
}
