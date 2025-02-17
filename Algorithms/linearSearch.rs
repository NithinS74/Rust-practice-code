fn main() {
    let num: i32 = 8;
    let array: [i32; 6] = [3, 4, 1, 8, 4, 5];
    linear_search(&array, num);
}

fn linear_search(arr: &[i32], key: i32) {
    for (index, &item) in arr.iter().enumerate() {
        if item == key {
            println!("{key} found at index {index}");
            return;
        }
    }
    println!("{key} not found");
    return;
}
