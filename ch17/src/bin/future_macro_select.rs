use futures::future::FutureExt;
use futures::select;

async fn async_identity_fn(arg: usize) -> usize {
    arg
}


#[tokio::main]
async fn main() {
    let res = select! {
        a_res = async_identity_fn(42).fuse() => a_res + 1,
        b_res = async_identity_fn(13).fuse() => b_res,
    };
    
    println!("Result: {}", res);
}