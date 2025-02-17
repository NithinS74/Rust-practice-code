fn main() {
    let x: Option<i32> = None;
    let x = nullchecker(x);
    println!("{}", x);
}

fn nullchecker(x: Option<i32>) -> i32 {
    match x {
        Some(y) => return y,
        None => return 0,
    }
}
