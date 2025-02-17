fn main() {
    let mut arr: [i32; 6] = [55, 42, 35, 12, 101, 77];
    println!("original array:\n{:?} ", arr);
    bubblesort(&mut arr);
}

fn bubblesort(arr: &mut [i32]) {
    let len = arr.len() - 1;
    for _i in 0..len {
        let mut p = 0;
        for j in 0..len {
            if arr[j] > arr[j + 1] {
                p += 1;
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
        println!("\nnumber of swaps this pass : {p}");
        println!("{:?} ", arr);
        if p == 0 {
            break;
        };
    }
    println!("\nSorted array:\n{:?} ", arr);
    println!("");
}
