use std::io;

fn main() {
    let mut height = String::new();
    println!("Enter your height in centimetres:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Input not a number");

    if height >= 150.0 && height <= 170.0 {
        println!("You are average height.");
    } else if height > 170.0 && height <= 195.0 {
        println!("You are tall.");
    } else if height < 150.0 {
        println!("You are short.");
    } else {
        println!("That height reading looks abnormal.");
    }
}