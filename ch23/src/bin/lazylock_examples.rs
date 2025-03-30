use std::sync::LazyLock;

fn main() {
    let lazy = LazyLock::new(|| 92);

    assert_eq!(LazyLock::force(&lazy), &92);
    assert_eq!(&*lazy, &92);
}
