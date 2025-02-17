fn find_min_max(arr: &[i32]) -> (i32, i32) {
    if arr.is_empty() {
        return (0, 0);
    }
    let mut min = arr[0];
    let mut max = arr[0];
    for &value in arr.iter() {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    (min, max)
}

fn main() {
    let numbers = [34, 67, 23, 89, 12, 45, 78];
    let (min, max) = find_min_max(&numbers);
    println!("min: {min}\nmax: {max}");
}
