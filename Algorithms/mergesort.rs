fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut merged = arr.to_vec();
    let left = &arr[..mid];
    let right = &arr[mid..];
    merge(left, right, &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], merged: &mut [i32]) {
    let (mut l, mut r, mut i) = (0, 0, 0);

    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            merged[i] = left[l];
            l += 1;
        } else {
            merged[i] = right[r];
            r += 1;
        }
        i += 1;
    }

    while l < left.len() {
        merged[i] = left[l];
        i += 1;
        l += 1;
    }
    while r < right.len() {
        merged[i] = right[r];
        i += 1;
        r += 1;
    }
}

fn main() {
    let mut numbers = [38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}
