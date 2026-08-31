fn main() {
    // Step 1. Declaring Variables
    let p: f64=210_000.0; // Initial TV value
    let r: f64=5.0; // Depreciation rate per annum
    let n: f64=3.0; // Number of years

    // Step 2. Making Calculation Statements
    let base = 1.0-(r/100.0);
    let a = p*base.powf(n);

    // Step 3. Printing Final Value after Depreciation
    println!("The value of the TV after {} years is N{}", n, a); // Depreciated Value
}