fn main() {
    // Step 1: Call the function and print the returned f64 value
    println!("pi value is {}", get_pi());
}

// Step 2: Define a function explicitly returning an f64 data type
fn get_pi() -> f64 {
    // Step 3: Declare explicit floating-point variables
    let a: f64 = 22.0;
    let b: f64 = 7.0;
    let c: f64 = a / b;
    
    // Step 4: Return the calculated value back to the caller
    return c;
}