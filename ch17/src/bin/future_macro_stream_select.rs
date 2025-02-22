use futures::{StreamExt, stream, stream_select};

#[tokio::main]
async fn main() {
    let endless_ints = |i| stream::iter(vec![i].into_iter().cycle()).fuse();

    let mut endless_numbers = stream_select!(endless_ints(1i32), endless_ints(2), endless_ints(3));
    match endless_numbers.next().await {
        Some(1) => println!("Got a 1"),
        Some(2) => println!("Got a 2"),
        Some(3) => println!("Got a 3"),
        _ => unreachable!(),
    }
}
