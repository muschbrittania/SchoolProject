fn main() {
    let mut numbers = Vec::new();
    loop {
        if numbers.len() >= 5 { break; }
        numbers.push(rand::random());
    }
    println!("Numbers: {:?}", numbers);
}
