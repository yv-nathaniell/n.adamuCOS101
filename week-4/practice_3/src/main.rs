use std::io;

fn main() {
    let mut base = String::new();
    println!("Enter the base:");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f32 = base.trim().parse().expect("Input not a number");

    let mut height = String::new();
    println!("Enter the height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Input not a number");

    if base > 0.0 {
        let area = (base * height) / 2.0;
        println!("The area of the triangle is: {}", area);
    }
}