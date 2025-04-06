use std::sync::atomic::AtomicI16;

fn main() {
    let some_var = AtomicI16::new(5);   
    assert_eq!(some_var.into_inner(), 5);

    // 错误，因为 `some_var` 已经被消费了
    // println!("some_var: {:?}", some_var);
}