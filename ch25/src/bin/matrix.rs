use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use rand::Rng;

// 模拟矩阵数据结构
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

fn main() {
    let matrix_size = 100;
    let thread_count = 4;

    // 创建一个随机矩阵
    let matrix1 = Arc::new(Mutex::new(Matrix {
        rows: matrix_size,
        cols: matrix_size,
        data: generate_matrix(matrix_size, matrix_size),
    }));

    // 创建另一个随机矩阵
    let matrix2 = Arc::new(Mutex::new(Matrix {
        rows: matrix_size,
        cols: matrix_size,
        data: generate_matrix(matrix_size, matrix_size),
    }));

    // 创建一个结果矩阵
    let result_matrix = Arc::new(Mutex::new(Matrix {
        rows: matrix_size,
        cols: matrix_size,
        data: vec![vec![0; matrix_size]; matrix_size],
    }));

    let barrier = Arc::new(Barrier::new(thread_count));

    let mut handles = vec![];

    // 创建线程,每个线程计算矩阵乘法的一部分
    for i in 0..thread_count {
        let matrix1_clone = Arc::clone(&matrix1);
        let matrix2_clone = Arc::clone(&matrix2);
        let result_matrix_clone = Arc::clone(&result_matrix);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            // 将矩阵分块处理。每个线程处理一部分矩阵
            for row in (i*matrix_size/thread_count)..((i+1)*matrix_size/thread_count) {
                for col in 0..matrix_size {
                    let matrix1_lock = matrix1_clone.lock().unwrap();
                    let matrix2_lock = matrix2_clone.lock().unwrap();
                    let mut result_matrix_lock = result_matrix_clone.lock().unwrap();

                    for k in 0..matrix_size {
                        result_matrix_lock.data[row][col] += matrix1_lock.data[row][k] * matrix2_lock.data[k][col];
                    }
                }
            }

            println!("线程 {}，完成部分计算，等待其他线程...", i);
            barrier_clone.wait(); // 等待所有线程到达屏障
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("矩阵乘法计算完成！");
}

// 辅助函数：生成随机矩阵
fn generate_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut r = rand::rng();
    (0..rows)
        .map(|_| (0..cols).map(|_| r.random_range(1..10)).collect())
        .collect()
}