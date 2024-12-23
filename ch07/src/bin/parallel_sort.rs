fn parallel_sort<T: Ord + Send>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let mid = slice.len() / 2;
    let (left, right) = slice.split_at_mut(mid);
    
    rayon::scope(|s| {
        s.spawn(|_| parallel_sort(left));
        s.spawn(|_| parallel_sort(right));
    });

    slice.sort();
}

fn main() {
    let mut data = vec![5, 2, 8, 1, 9, 4, 7, 3, 6];
    parallel_sort(&mut data);
    println!("{:?}", data); // 输出：[1, 2, 3, 4, 5, 6, 7, 8, 9]
}