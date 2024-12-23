pub fn merge_sort_serial<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort_serial(&mut arr[..mid]);
    merge_sort_serial(&mut arr[mid..]);

    // merge_bad(&mut arr[..mid],&mut arr[mid..]);

    merge(arr);
}

fn merge<T: Ord + Clone>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[allow(dead_code)]
fn merge_bad<T: Ord + Clone>(left: &mut [T], right: &mut [T]) {
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
    left.iter_mut()
        .chain(right.iter_mut())
        .zip(sorted.into_iter())
        .for_each(|(slot, value)| *slot = value);
}
