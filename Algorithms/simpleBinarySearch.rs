fn main() {
    let num: i32 = 8;
    let array: [i32; 6] = [1, 4, 5, 7, 8, 11];
    binary_search(&array, num);
}

fn binary_search(arr: &[i32], key: i32) {
    let mut high: usize = arr.len() - 1;
    let mut low: usize = 0;
    let mut mid: usize;
    loop {
        mid = low + (high - low) / 2;
        if arr[mid] == key {
            println!("key found at index {mid}");
            return;
        } else if low == high {
            println!("Key not found!!");
            return;
        } else if arr[mid] > key {
            high = mid - 1;
        } else if arr[mid] < key {
            low = mid + 1;
        }
    }
}
