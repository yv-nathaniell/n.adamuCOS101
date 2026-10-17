fn main() {
    // Name vector[cite: 8]
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    // Age vector[cite: 8]
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];
    
    print!("\nAge allocation:\n");
    
    //loop to iterate elements in vector[cite: 8]
    for i in 0..age.len() {
        // iterating through i on the vector[cite: 8]
        print!("{} is {} years old\n", name[i], age[i]);
    }
}
