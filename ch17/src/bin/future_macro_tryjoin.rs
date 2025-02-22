use futures::try_join;

#[tokio::main]
async fn main() {
    let a = async { Ok::<i32, i32>(1) };
    let b = async { Ok::<i32, i32>(2) };

    assert_eq!(try_join!(a, b), Ok((1, 2)));

    // `try_join!` 是可变参数的，因此您可以传递任意数量的 futures
    let c = async { Ok::<i32, i32>(3) };
    let d = async { Ok::<i32, i32>(4) };
    let e = async { Ok::<i32, i32>(5) };

    assert_eq!(try_join!(c, d, e), Ok((3, 4, 5)));

    let a = async { Ok::<i32, i32>(1) };
    let b = async { Err::<u64, i32>(2) };

    assert_eq!(try_join!(a, b), Err(2));
}
