fn main() {
    let x = (5, -2);
    conditional_match(x);
}

fn conditional_match(x: (i32, i32)) {
    match x {
        (x, y) if x > 0 && y > 0 => println!("Positive"),
        (x, y) if x < 0 && y < 0 => println!("Negetive"),
        (0, 0) => println!("Zero"),
        _ => println!("Mixed"),
    }
}
