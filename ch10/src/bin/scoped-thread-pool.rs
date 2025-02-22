use scoped_thread_pool::Pool;

fn main() {
    let mut buf = [0, 0, 0, 0];

    let pool = Pool::new(4);
    pool.scoped(|scope| {
        for i in &mut buf {
            scope.execute(move || *i += 1);
        }
    });

    assert_eq!(&buf, &[1, 1, 1, 1]);
}
