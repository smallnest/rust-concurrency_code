use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();

fn main() {
    // 中毒
    let handle = thread::spawn(|| {
        INIT.call_once(|| panic!());
    });
    assert!(handle.join().is_err());

    // 后续的调用会直接返回panic
    let handle = thread::spawn(|| {
        INIT.call_once(|| {});
    });
    assert!(handle.join().is_err());

    // 调用 call_once_force 会执行初始化函数，并重置中毒状态
    INIT.call_once_force(|state| {
        assert!(state.is_poisoned());
    });

    // 一旦成功调用一次，就不会再传播中毒状态
    INIT.call_once(|| {});

    println!("exit!");
}
