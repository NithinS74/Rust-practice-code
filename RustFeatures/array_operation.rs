fn main() {
    let arr = [1, 2, 3, 4, 5];
    if let Some(out) = arrays(&arr) {
        println!("{:?}", out);
    } else {
        println!("None");
    }
}

#[derive(Debug)]
struct output {
    largest: i32,
    smallest: i32,
    average: i32,
}

fn arrays(arr: &[i32]) -> Option<output> {
    if arr.len() == 0 {
        return None;
    }
    let sum: i32 = arr.iter().sum();
    let average: i32 = sum / arr.len() as i32;
    Some(output {
        largest: arr.iter().max().unwrap().clone(),
        smallest: arr.iter().min().unwrap().clone(),
        average,
    })
}
