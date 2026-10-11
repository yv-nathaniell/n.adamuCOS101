fn main() {
    // Step 1: Define an array of 5 string slices
    let city_arr: [&str; 5] = ["Abuja", "Portharcourt", "Maiduguri", "Kano", "Lagos"];
    
    println!("array is {:?}", city_arr);
    println!("array size is: {}", city_arr.len());
    
    // Step 2: Iterate over the array elements strictly by their index range
    for index in 0..5 {
        println!("City index {} is located in: {}", index, city_arr[index]);
    }
}