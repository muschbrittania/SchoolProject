fn main() {
    let mut max = 0;
    loop {
        println!("Enter number:");
        let input = std::io::stdin().read_line().unwrap();
        if input.trim().eq("quit") {
            break;
        }
        let num: i32 = input.trim().parse::<i32>().expect("Please enter a valid integer");
        max = max.max(num);
    }
    println!("The maximum value is: {}", max);
}
