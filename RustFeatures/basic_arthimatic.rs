fn main() {
    let a = 4;
    let b = 2;
    let out = operations(a, b);
    println!("{:?}", out);
}

#[derive(Debug)]
struct arthimatic_output {
    sum: i32,
    product: i32,
    diffrence: i32,
    quotient: Option<i32>,
    reminder: Option<i32>,
}

fn operations(a: i32, b: i32) -> arthimatic_output {
    let quotient: Option<i32>;
    let reminder: Option<i32>;
    if b == 0 {
        quotient = None;
        reminder = None;
    } else {
        quotient = Some(a / b);
        reminder = Some(a % b);
    }
    arthimatic_output {
        sum: (a + b),
        product: (a * b),
        diffrence: (a - b),
        quotient,
        reminder,
    }
}
