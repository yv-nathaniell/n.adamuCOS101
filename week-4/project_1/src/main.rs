use std::io;

fn main() {
    let mut a = String::new();
    println!("Enter a:");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Input not a number");

    let mut b = String::new();
    println!("Enter b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Input not a number");

    let mut c = String::new();
    println!("Enter c:");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Input not a number");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
}