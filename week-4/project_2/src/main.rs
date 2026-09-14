use std::io;

fn main() {
    let mut experienced = String::new();
    println!("Are you experienced? (yes/no):");
    io::stdin().read_line(&mut experienced).expect("Failed to read input");
    let experienced = experienced.trim().to_lowercase();

    let mut age = String::new();
    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Input not an integer");

    let incentive: u32;

    if experienced == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // ages 28 and 29 fall through every stated rule — the brief has a gap here
            incentive = 0;
        }
    } else {
        incentive = 100_000;
    }

    println!("Your annual incentive is: N{}", incentive);
}