use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream1 = stream::iter(vec![1, 2, 3]);
    let mut stream2 = stream::iter(vec![4, 5, 6]);

    let next = tokio::select! {
        v = stream1.next() => v.unwrap(),
        v = stream2.next() => v.unwrap(),
    };

    assert!(next == 1 || next == 4);
}