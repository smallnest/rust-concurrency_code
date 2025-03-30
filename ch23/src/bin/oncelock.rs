use std::sync::OnceLock;

static CELL: OnceLock<usize> = OnceLock::new();

fn main() {
    // OnceLock还没有被写入。
    assert!(CELL.get().is_none());

    // 启动一个线程并写入OnceLock。
    std::thread::spawn(|| {
        let value = CELL.get_or_init(|| 12345);
        assert_eq!(value, &12345);
    })
    .join()
    .unwrap();

    // `OnceLock`现在包含值。
    assert_eq!(CELL.get(), Some(&12345),);
}
