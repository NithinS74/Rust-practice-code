fn largest<T: PartialOrd>(array: &[T]) -> &T {
    let mut l = 0;
    for i in 0..array.len() {
        if array[i] > array[l] {
            l = i;
        }
    }
    return &array[l];
}

fn main() {
    let x = [1, 2, 3, 4, 5];
    let y = ['a', 'b', 'c', 'd'];
    println!("{}", largest(&x));
    println!("{}", largest(&y));
}
