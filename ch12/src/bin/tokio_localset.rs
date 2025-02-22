use std::rc::Rc;
use tokio::task;

#[tokio::main]
async fn main() {
    let nonsend_data = Rc::new("my nonsend data...");

    // Construct a local task set that can run `!Send` futures.
    let local = task::LocalSet::new();

    // Run the local task set.
    local.run_until(async move {
        let nonsend_data = nonsend_data.clone();
        // `spawn_local` ensures that the future is spawned on the local
        // task set.
        task::spawn_local(async move {
            println!("{}", nonsend_data);
            // ...
        }).await.unwrap();
    }).await;
}

fn wrong() {
        // // `Rc` does not implement `Send`, and thus may not be sent between
    // // threads safely.
    // let nonsend_data = Rc::new("my nonsend data...");

    //     let nonsend_data = nonsend_data.clone();
    //     // Because the `async` block here moves `nonsend_data`, the future is `!Send`.
    //     // Since `tokio::spawn` requires the spawned future to implement `Send`, this
    //     // will not compile.
    //     tokio::spawn(async move {
    //         println!("{}", nonsend_data);
    //         // ...
    //     }).await.unwrap();
}