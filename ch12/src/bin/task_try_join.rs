async fn do_stuff_async() -> Result<(), &'static str> {
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

async fn more_async_work() -> Result<(), &'static str> {
    Err("more work failed")
}

#[tokio::main]
async fn main() {
    let res = tokio::try_join!(
        do_stuff_async(),
        more_async_work());

    match res {
         Ok((first, second)) => {
             println!("first: {:?}, second: {:?}", first, second);
         }
         Err(err) => {
            println!("processing failed; error = {}", err);
         }
    }
}