fn main() {
    let a = [1, 2, 3, 4];
    let v_iter = a.iter();
    let s: i32 = v_iter.sum();
    let v_iter = a.iter();
    let clo = |num: i32| {
        println!("work");
    };
    clo(4);
    println!("{:?}\n{:?}\nsum = {}", a, v_iter, s);
}
