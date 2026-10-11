use std::io;

// Step 1: Define independent calculation functions for every shape
fn calc_trapezium() {
    let mut input = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let height: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base1:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let base1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter base2:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let base2: f64 = input.trim().parse().expect("Invalid input");

    // Formula: height / 2 * (base1 + base2)
    let area = (height / 2.0) * (base1 + base2);
    println!("Area of the Trapezium: {}", area);
}

fn calc_rhombus() {
    let mut input = String::new();
    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let d1: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let d2: f64 = input.trim().parse().expect("Invalid input");

    // Formula: 1/2 * diagonal1 * diagonal2
    let area = 0.5 * d1 * d2;
    println!("Area of the Rhombus: {}", area);
}

fn calc_parallelogram() {
    let mut input = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let base: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter altitude:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let altitude: f64 = input.trim().parse().expect("Invalid input");

    // Formula: base * altitude
    let area = base * altitude;
    println!("Area of the Parallelogram: {}", area);
}

fn calc_cube() {
    let mut input = String::new();
    println!("Enter the length of the side:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let side: f64 = input.trim().parse().expect("Invalid input");

    // Formula: 6 * side * side
    let area = 6.0 * (side * side);
    println!("Surface Area of the Cube: {}", area);
}

fn calc_cylinder() {
    let mut input = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let radius: f64 = input.trim().parse().expect("Invalid input");

    input.clear();
    println!("Enter height:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let height: f64 = input.trim().parse().expect("Invalid input");

    // Formula: pi * radius * radius * height
    let pi: f64 = 22.0 / 7.0; 
    let volume = pi * radius * radius * height;
    println!("Volume of the Cylinder: {}", volume);
}

fn main() {
    // Step 2: Render the selection menu to the user
    println!("Welcome to the Shape Calculator!");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");
    println!("Select a shape to calculate (1-5):");

    // Step 3: Capture the choice selection
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid choice");

    // Step 4: Route the execution sequence to the corresponding shape function
    if choice == 1 {
        calc_trapezium();
    } else if choice == 2 {
        calc_rhombus();
    } else if choice == 3 {
        calc_parallelogram();
    } else if choice == 4 {
        calc_cube();
    } else if choice == 5 {
        calc_cylinder();
    } else {
        println!("Invalid selection. Please run the program again.");
    }
}