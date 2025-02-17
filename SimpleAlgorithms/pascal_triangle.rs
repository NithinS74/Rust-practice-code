pub fn pascal_triangle_print() {
    let n = 4;
    for i in 1..=n {
        for _j in i..=n {
            print!(" ");
        }
        let mut coefficient = 1;
        for k in 1..=i {
            print!("{coefficient} ");
            coefficient = (coefficient * (i - k)) / k;
        }
        print!("\n");
    }
}
