fn main() {
    let mut arr: [i32; 6] = [32, 44, 23, 55, 12, 5];
    println!("Unsorted: {:?}", arr);
    selection_sort(&mut arr);
    println!("sorted:   {:?}", arr);
}

fn selection_sort(arr: &mut [i32]) {
    let mut asorted: usize = 0;

    while asorted < arr.len() {
        let mut smallest = arr[asorted];
        let mut lindex = asorted;
        for (index, &iteam) in arr[asorted..].iter().enumerate() {
            if smallest > iteam {
                smallest = iteam;
                lindex = index + asorted;
            }
        }
        if asorted != lindex {
            arr[lindex] = arr[asorted];
            arr[asorted] = smallest;
        }
        asorted += 1;
    }
}
