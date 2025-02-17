fn main() {
    let mut x: Vec<i32> = (10..=50).step_by(10).collect();
    let mut y = &x[1..=3].to_vec();
    for (i, &mut v) in y.iter() {}
    print!("{:?} {:?}", x, y);
}
