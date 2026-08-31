fn main() {
	// Step 1. Declaring Variables
	let p: f64=520_000_000.0; // Principal value
	let r: f64=10.0; // Rate per annum value
	let n: f64=5.0; // Number of years value

	// Step 2. Making Calculation Statements
	let base = 1.0+(r/100.0);
	let a = p*base.powf(n);
	let ci = a-p;

	// Step 3. Printing Amount then Compound Interest
	println!("The total amount after {} years is N{}", n, a); // Amount Value
	println!("The Compound Interest is N{}", ci); // Compound Interest Value
}
