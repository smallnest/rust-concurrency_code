use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct ExpensiveResource { value: i32 }
static RESOURCE: AtomicPtr<ExpensiveResource> = AtomicPtr::new(ptr::null_mut());

fn get_or_init_resource() -> &'static ExpensiveResource {
    // 尝试快速路径：如果已初始化，直接返回
    let mut ptr = RESOURCE.load(Ordering::Acquire); // Acquire 确保看到已初始化的资源
    if ptr.is_null() {
        // 慢路径：尝试初始化
        let new_resource = Box::into_raw(Box::new(ExpensiveResource { value: 42 }));
        // AcqRel: 如果成功，Release 确保资源对其他线程可见；Acquire 与其他线程的初始化同步。
        // Acquire (failure): 如果失败，Acquire 确保能看到成功初始化的线程设置的指针。
        match RESOURCE.compare_exchange(ptr::null_mut(), new_resource, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => {
                // 我们成功初始化了它
                ptr = new_resource;
            }
            Err(current_ptr) => {
                // 其他线程在我们之前初始化了它
                // 释放我们创建但未使用的资源
                unsafe { drop(Box::from_raw(new_resource)); }
                ptr = current_ptr; // 使用已存在的指针
            }
        }
    }
    // Safety: 一旦初始化，指针指向的静态资源就不会改变或被释放。
    // 这要求 ExpensiveResource 本身是 Sync 的。
    unsafe { &*ptr }
}

fn main() {
    let res = get_or_init_resource();
    println!("Resource value: {}", res.value);
}