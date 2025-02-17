struct co_ordinates {
    x: i32,
    y: i32,
}

impl co_ordinates {
    fn caluclate_from_origin(&self) {
        let d = ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt();
        print!("{d}");
    }
}

fn main() {
    let p = co_ordinates { x: 2, y: 3 };
    p.caluclate_from_origin();
}
