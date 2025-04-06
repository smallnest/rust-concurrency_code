use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::atomic::compiler_fence;

static mut IMPORTANT_VARIABLE: usize = 0;
static IS_READY: AtomicBool = AtomicBool::new(false);

fn main() {
    unsafe { IMPORTANT_VARIABLE = 42 };
    // 将之前的写入标记为与未来的 relaxed stores 一起释放
    compiler_fence(Ordering::Release);
    IS_READY.store(true, Ordering::Relaxed);

    // 调用信号处理函数来测试同步
    signal_handler();
}

fn signal_handler() {
    if IS_READY.load(Ordering::Relaxed) {
        // 获取那些通过我们读取的 relaxed stores 释放的写入
        compiler_fence(Ordering::Acquire);
        assert_eq!(unsafe { IMPORTANT_VARIABLE }, 42);
    }
}
