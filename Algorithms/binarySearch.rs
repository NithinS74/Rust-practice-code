fn binary_search(arr: &[i32], key: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    while low <= high {
        let mid: usize = low + ((high) - (low) / 2);
        match arr[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_test() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let arr1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(binary_search(&arr, 4), Some(3));
        assert_eq!(binary_search(&arr1, 9), Some(8));
    }

    #[test]
    fn notfound_test() {
        let arr9 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let arr10 = vec![1, 10, 100, 1000, 10000, 100000, 1000000];

        assert_eq!(binary_search(&arr10, 242424), None);
        assert_eq!(binary_search(&arr9, 242424), None);
    }

    #[test]
    fn edge_cases() {
        let arr3 = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
        let arr4 = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20];

        assert_eq!(binary_search(&arr3, 5), Some(0));
        assert_eq!(binary_search(&arr4, 20), Some(9));
    }
}
