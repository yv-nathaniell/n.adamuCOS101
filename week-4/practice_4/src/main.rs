use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim();

    let mut age = String::new();
    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Input not an integer");

    if age >= 18 {
        println!("Welcome to the party, {}!", name);
    } else {
        println!("Oops, {}, you are not of age to enter the party.", name);
    }
}