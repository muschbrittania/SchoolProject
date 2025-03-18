fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let target_number = 7;
    for number in numbers.iter().filter(|&&x| x == target_number) {
        println!("Found the number {}", number);
    }
}
