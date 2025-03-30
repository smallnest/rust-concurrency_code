use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use rand::Rng;

// 模拟水分子（H2O）
struct H2O {
    hydrogen_barrier: Arc<Barrier>, // 用于同步两个氢原子的屏障
    oxygen_barrier: Arc<Barrier>, // 用于同步一个氧原子的屏障
    molecule_barrier: Arc<Barrier>, // 用于同步水分子的屏障
    molecule_count: Mutex<usize>, // 用于计数创建的水分子数量
}

// 每个水分子由两个氢原子和一个氧原子组成
// 使用栅栏来同步氢原子和氧原子的创建
impl H2O {
    fn new() -> Self {
        H2O {
            hydrogen_barrier: Arc::new(Barrier::new(2)),
            oxygen_barrier: Arc::new(Barrier::new(1)),
            molecule_barrier: Arc::new(Barrier::new(3)),
            molecule_count: Mutex::new(0),
        }
    }

    // 模拟氢原子和氧原子的创建
    // 每个氢原子需要等待另一个氢原子和氧原子的创建
    fn hydrogen(&self) {
        self.hydrogen_barrier.wait();
        self.molecule_barrier.wait();
    }

    // 每个氧原子需要等待两个氢原子的创建
    fn oxygen(&self) {
        self.oxygen_barrier.wait();
        self.molecule_barrier.wait();
        let mut count = self.molecule_count.lock().unwrap();
        *count += 1;
        println!("Molecule {} created", *count);
    }
}

fn main() {
    let h2o = Arc::new(H2O::new());
    let mut handles = vec![];

    // 创建氢原子线程, 10个线程，每个线程随机创建20个氢原子
    for _ in 0..10 {
        let h2o_clone = Arc::clone(&h2o);
        handles.push(thread::spawn(move || {
            let mut r = rand::rng();
            for _ in 0..20 { 
                h2o_clone.hydrogen();
                thread::sleep(std::time::Duration::from_millis(r.random::<u64>() % 1000));
            }
        }));
    }

    // 创建氧原子线程, 5个线程，每个线程随机创建100个氧原子
    for _ in 0..5 {
        let h2o_clone = Arc::clone(&h2o);
        handles.push(thread::spawn(move || {
            let mut r = rand::rng();
            for _ in 0..20 {   
                h2o_clone.oxygen();
                thread::sleep(std::time::Duration::from_millis(r.random::<u64>() % 1000));
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    h2o.molecule_barrier.wait(); // 等待所有线程完成

    println!("All molecules created");
}