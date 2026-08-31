fn main() {
    // Step 1. Declaring Variables for Item Amounts
    let item1: f64=450_000.00; // Toshiba amount
    let item2: f64=1_500_000.00; // Mac amount
    let item3: f64=750_000.00; // HP amount
    let item4: f64=2_850_000.00; // Dell amount
    let item5: f64=250_000.00; // Acer amount

    // Step 2. Making Calculation Statements
    let sum = item1+item2+item3+item4+item5;
    let average = sum/5.0;

    // Step 3. Printing Sum then Average
    println!("The total sum of sales is N{}", sum); // Total Sum Value
    println!("The average sales value is N{}", average); // Average Value
}