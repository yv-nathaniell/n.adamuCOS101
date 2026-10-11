fn main() {
    // Step 1: Declare a mutable integer variable
    let mut num: i32 = 5;
    
    // Step 2: Pass by reference using &mut to lend the memory address
    mutate_num_to_zero(&mut num);
    
    // Step 3: Print the original variable to see the permanent change
    println!("The value of num is:{}", num);
}

// Step 4: Function receives the reference pointer
fn mutate_num_to_zero(param_num: &mut i32) {
    // Step 5: Dereference using '*' to edit the underlying original value
    *param_num = *param_num * 0;
    println!("param_num value is: {}", param_num);
}