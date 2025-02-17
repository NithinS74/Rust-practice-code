fn main() {
    describe(55, "hello");
    describe("hello world!!", [1, 2, 3]);
}

fn describe(a: impl std::fmt::Debug, b: impl std::fmt::Debug) {
    println!("{:?} {:?}", a, b);
}
