use std::io;

// Step 1: Define a function taking two 32-bit integer parameters
fn add(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum of A and B = {}", sum);
}

fn main() {
    // Step 2: Read the first integer parameter from the user
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a: i32 = input1.trim().parse().expect("Invalid input");
    
    // Step 3: Read the second integer parameter from the user
    let mut input2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b: i32 = input2.trim().parse().expect("Invalid input");
    
    // Step 4: Pass the arguments into the function by value
    add(a, b);
}