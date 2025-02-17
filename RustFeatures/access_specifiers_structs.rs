mod test {
    pub enum shape {
        square,
        circle,
        triangle,
    }
}
fn main() {
    let x = test::shape::circle;
}
