// Define a struct to represent a person with name and age fields
struct Person {
    name: String,
    age: u32,
}

// Create a new instance of the `Person` struct
let mut john = Person { name: "John".to_string(), age: 30 };

// Print the value of the `name` field for the `john` variable
println!("{}", john.name);

// Change the value of the `age` field for the `john` variable
john.age = 31;

// Print the updated value of the `age` field for the `john` variable
println!("{}", john.age);
