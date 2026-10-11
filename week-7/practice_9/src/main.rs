fn main() {
    // Step 1: Declare an integer array
    let arr: [i32; 4] = [10, 20, 30, 40];
    
    println!("array is {:?}", arr);
    println!("array size is: {}", arr.len());
    
    // Step 2: Traverse the array safely using the iter() method
    for val in arr.iter() {
        println!("value is: {}", val);
    }
}