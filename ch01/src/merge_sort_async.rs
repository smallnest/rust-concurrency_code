pub async fn merge_sort_async<T: Ord + Send + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    // 使用 async/await 来并发处理左右半部分的排序
    let ((), ()) = futures::join!(
        Box::pin(merge_sort_async(left)),
        Box::pin(merge_sort_async(right)),
    );

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