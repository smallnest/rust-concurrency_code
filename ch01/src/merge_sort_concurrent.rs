use std::thread;


pub fn merge_sort_concurrent<T: Ord + Send + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    thread::scope(|s| {
        // 在scope内创建线程并发执行左右半部分的排序
        s.spawn(|| merge_sort_concurrent(left));
        s.spawn(|| merge_sort_concurrent(right));
    });

    merge(left, right);
}

fn merge<T: Ord + Clone>(left: &mut [T], right: &mut [T]) {
    let mut sorted = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            sorted.push(left[i].clone());
            i += 1;
        } else {
            sorted.push(right[j].clone());
            j += 1;
        }
    }

    // 将剩余的元素添加到sorted中
    sorted.extend_from_slice(&left[i..]);
    sorted.extend_from_slice(&right[j..]);

    // 将结果拷贝回原始数组
    left.iter_mut().chain(right.iter_mut()).zip(sorted.into_iter())
        .for_each(|(slot, value)| *slot = value);
}
