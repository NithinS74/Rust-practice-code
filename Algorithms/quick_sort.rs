fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let mid = arr.len() / 2;
    let temp = arr[arr.len() - 1];
    arr[arr.len() - 1] = arr[mid];
    arr[mid] = temp;

    let pivot = arr[arr.len() - 1];
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i += 1;
        }
    }
    let j = arr.len() - 1;
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
    i
}

fn main() {
    let mut numbers = [38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}
