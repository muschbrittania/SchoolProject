fn main() {
    // Example of using a vector as an iterator in Rust

    let numbers = vec![1, 2, 3, 4, 5];
    for number in &numbers {
        println!("The current number is: {}", number);
    }
}
