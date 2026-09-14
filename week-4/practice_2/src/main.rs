use std::io;

fn main() {
    let mut a = String::new();
    println!("Enter side a:");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f32 = a.trim().parse().expect("Input not a number");

    let mut b = String::new();
    println!("Enter side b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f32 = b.trim().parse().expect("Input not a number");

    let mut c = String::new();
    println!("Enter side c:");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f32 = c.trim().parse().expect("Input not a number");

    let s: f32 = (a + b + c) / 2.0;
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    println!("The area of the triangle is: {}", area);
}