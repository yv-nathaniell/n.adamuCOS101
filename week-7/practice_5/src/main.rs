fn main() {
    // Step 1: Declare a standard integer variable
    let num: i32 = 5;
    
    // Step 2: Pass by value. The function receives a separate copy
    mutate_num_to_zero(num);
    
    // Step 3: Print the original variable to confirm it is completely untouched
    println!("The value of no is:{}", num);
}

// Step 4: Function takes ownership of the copy and mutates it locally
fn mutate_num_to_zero(mut param_num: i32) {
    param_num = param_num * 0;
    println!("param_num value is:{}", param_num);
}