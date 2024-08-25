#![feature(test)]
extern crate test;

use ch01::{merge_sort_concurrent, merge_sort_serial,merge_sort_async};
use rand::prelude::*;
use tokio::runtime::Runtime;

use test::Bencher;

fn generate_random_vec(size: i32) -> Vec<i32> {
    let seed: u64 = 12345; 
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.gen_range(0..size)).collect()
}

#[bench]
fn benchmark_merge_sort_concurrent(b: &mut Bencher) {
    let arr = generate_random_vec(100);

    b.iter(|| {
        let mut data = test::black_box(arr.clone());
        merge_sort_concurrent(&mut data);
    })
}

#[bench]
fn benchmark_merge_sort_serial(b: &mut Bencher) {
    let arr = generate_random_vec(10000);

    b.iter(|| {
        let mut data = test::black_box(arr.clone());
        merge_sort_serial(&mut data);
    })
}

#[bench]
fn benchmark_merge_sort_async(b: &mut Bencher) {
    let arr = generate_random_vec(10000);
    let rt = Runtime::new().unwrap();

    b.iter(|| {
        let mut data = test::black_box(arr.clone());
        rt.block_on(merge_sort_async(&mut data));
    })
}
