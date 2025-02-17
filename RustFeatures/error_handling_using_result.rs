fn main() {
    let a = 5;
    let b = 0;
    match divide(a, b) {
        Ok(value) => println!("recived value: {value}"),
        Err(value) => println!("recived value: {value}"),
    }
}

fn divide(a: i32, b: i32) -> Result<f64, &'static str> {
    if b != 0 {
        let c: f64 = a as f64 / b as f64;
        return Ok(c);
    } else {
        return Err("cannot divide by 0");
    }
}
