use std::io;

fn main() {
    let mut lower = String::new();
    println!("Enter the lower bound:");
    io::stdin().read_line(&mut lower).expect("Failed to read input");
    let lower_bound: i32 = lower.trim().parse().expect("Input not an integer");

    let mut upper = String::new();
    println!("Enter the upper bound:");
    io::stdin().read_line(&mut upper).expect("Failed to read input");
    let upper_bound: i32 = upper.trim().parse().expect("Input not an integer");

    for x in lower_bound..upper_bound {
        println!("{}", x);
    }
}