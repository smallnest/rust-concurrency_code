use futures_time::channel;
use futures_time::prelude::*;
use futures_time::time::Duration;

#[tokio::main]
async fn main() {
    let (send, mut recv) = channel::bounded::<()>(1); // create a new send/receive pair
  
    let value = async { "meow" }
        .delay(Duration::from_millis(100))
        .timeout(recv.recv())
        .await;

    assert_eq!(value.unwrap(), "meow");
}
