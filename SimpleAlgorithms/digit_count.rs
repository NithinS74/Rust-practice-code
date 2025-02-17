pub fn digit_count() {
    let n = 20;
    let (m4, m7) = count(n);
    println!("no of 4s: {m4}\nno of 7s: {m7}\n in total: {}", (m4 + m7));
}

fn count(n: u32) -> (u32, u32) {
    let mut m4: u32 = 0;
    let mut m7: u32 = 0;
    for i in 1..=n {
        let mut k = i;
        while k != 0 {
            match k % 10 {
                4 => m4 += 1,
                7 => m7 += 1,
                _ => {}
            };
            k /= 10;
        }
    }
    return (m4, m7);
}
