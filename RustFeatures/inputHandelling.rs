use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("readline error!");
    let m = n.trim().parse::<i32>().expect("parse error");
    println!("string: {}\n int: {}", n, m);
}
