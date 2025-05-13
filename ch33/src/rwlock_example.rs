use std::sync::RwLock;

pub struct SharedResource {
    data: RwLock<i32>,
}

impl SharedResource {
    pub fn new(initial_value: i32) -> Self {
        SharedResource {
            data: RwLock::new(initial_value),
        }
    }

    pub fn read_data(&self) -> i32 {
        let data = self.data.read().unwrap();
        *data
    }

    pub fn write_data(&self, value: i32) {
        let mut data = self.data.write().unwrap();
        *data = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_rwlock_deadlock() {
        let resource = Arc::new(RwLock::new(0));

        let r1 = Arc::clone(&resource);
        let r2 = Arc::clone(&resource);

        let handle1 = thread::spawn(move || {
            let _lock1 = r1.read().unwrap();
            // 模拟一些工作
            thread::sleep(std::time::Duration::from_millis(50));
            let _lock2 = r1.write().unwrap();
        });

        let handle2 = thread::spawn(move || {
            let _lock2 = r2.write().unwrap();
            // 模拟一些工作
            thread::sleep(std::time::Duration::from_millis(50));
        });

        handle1.join().unwrap();
        handle2.join().unwrap();
    }
}
