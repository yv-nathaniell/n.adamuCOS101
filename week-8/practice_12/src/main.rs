fn main() {
    // mutable array[cite: 20]
    let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array = {:?}", colors);
    
    // mutable slice[cite: 20]
    let sliced_colors = &mut colors[1..3];
    println!("First slice = {:?}", sliced_colors);
    
    // change the value of the original slice at the first index[cite: 20]
    sliced_colors[1] = "purple";
    println!("Changed slice = {:?}", sliced_colors);
}
