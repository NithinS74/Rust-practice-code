fn main() {
    let x = 94;
    matchrange(x);
}

fn matchrange(x: i32) {
    match x {
        90..100 => println!("A"),
        80..89 => println!("B"),
        70..79 => println!("C"),
        _ => println!("F"),
    }
}
