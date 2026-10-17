fn main() {
    // Using Vec::new()[cite: 5]
    let v: Vec<i64> = Vec::new();
    // printing the size of vector[cite: 5]
    println!("\nThe length of Vec::new is: {}", v.len());
    
    // Using macro[cite: 5]
    let v2 = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];
    // printing the size of vector[cite: 5]
    println!("\nThe length of vec macro is: {}", v2.len());
}
