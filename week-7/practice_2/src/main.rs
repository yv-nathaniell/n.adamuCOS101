use std::io;

// Step 1: Define the character checker function
fn checker() {
    let mut input = String::new();
    println!("Enter a character: ");
    
    // Step 2: Read and parse the user input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch: char = input.trim().parse().expect("Invalid input");
    
    // Step 3: Evaluate if the character falls within the digit range
    if ch >= '0' && ch <= '9' {
        println!("Character '{}' is a digit", ch);
    } else {
        println!("Character '{}' is not a digit", ch);
    }
}

fn main() {
    // Step 4: Display welcome message and invoke the checker
    println!("Welcome! This program checks whether a character variable contains a digit or not");
    checker();
}