pub fn array_insertion() {
    let mut x = [0; 10];
    println!("{:?}", x);
    insert_at(&mut x, 8, 6);
    insert_at(&mut x, 4, 5);
    println!("{:?}", x);
}

fn insert_at(x: &mut [i32], index: usize, value: i32) {
    for i in (index..x.len()).rev() {
        x[i] = x[i - 1];
    }
    x[index] = value;
}
