fn main() {
    let n: u8 = 233;
    match n {
        1..=10 => println!("small"),
        11..=100 => println!("medium"),
        _ => println!("large"),
    }
}
