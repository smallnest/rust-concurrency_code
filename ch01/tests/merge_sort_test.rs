#[cfg(test)]
mod tests {
    use ch01::{merge_sort_concurrent, merge_sort_serial, merge_sort_async};

    #[test]
    fn test_merge_sort_concurrent() {
        let mut arr = vec![4, 2, 3, 1];
        merge_sort_concurrent(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4]);

        let mut arr = vec![9, 5, 7, 1, 3];
        merge_sort_concurrent(&mut arr);
        assert_eq!(arr, vec![1, 3, 5, 7, 9]);

        // Test with empty array
        let mut arr: Vec<i32> = vec![];
        merge_sort_concurrent(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_merge_sort_serial() {
        let mut arr = vec![4, 2, 3, 1];
        merge_sort_serial(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4]);

        let mut arr = vec![9, 5, 7, 1, 3];
        merge_sort_serial(&mut arr);
        assert_eq!(arr, vec![1, 3, 5, 7, 9]);

        // Test with empty array
        let mut arr: Vec<i32> = vec![];
        merge_sort_serial(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[tokio::test]
    async fn test_merge_sort_async() {
        // Test with an empty array
        let mut arr: [i32; 0] = [];
        merge_sort_async(&mut arr).await;
        assert_eq!(arr, []);

        // Test with an array of one element
        let mut arr = [5];
        merge_sort_async(&mut arr).await;
        assert_eq!(arr, [5]);

        // Test with an array of two elements
        let mut arr = [5, 2];
        merge_sort_async(&mut arr).await;
        assert_eq!(arr, [2, 5]);

        // Test with an array of multiple elements
        let mut arr = [5, 2, 9, 1, 7];
        merge_sort_async(&mut arr).await;
        assert_eq!(arr, [1, 2, 5, 7, 9]);

        // Test with a large array
        let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        merge_sort_async(&mut arr).await;
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
