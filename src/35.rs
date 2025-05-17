fn main() {
    // Example Rust code with variables and basic operations

    let num1 = 5;
    let num2 = 3;

    println!("The sum of {} and {}: {}", num1, num2, num1 + num2);

    let grade: u8 = 90;
    let score = 85;

    if grade >= 60 {
        println!("You got an A!");
    } else if grade >= 70 {
        println!("You got a B!");
    } else {
        println!("Sorry, but you failed.");
    }

    let current_time = SystemTime::now();
    let milliseconds = (current_time - previous_time).to_millis();

    println!("The time is {}ms", milliseconds);
}
