fn main() {
    let src = [1, 2, 3, 4];
    let mut dst = [0; 4]; // Destination array initialized with zeros

    // Copying elements from src to dst
    dst.copy_from_slice(&src);

    println!("Source: {:?}", src);
    println!("Destination: {:?}", dst);
}
