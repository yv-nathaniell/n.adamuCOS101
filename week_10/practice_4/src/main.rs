fn main() {
    // a list of nos
    let v = vec![15, 25, 35, 45, 55];
    print_vector(v.clone()); // We clone v to avoid giving up ownership permanently
    println!("{}", v[0]); // this line gives error if v was moved entirely
}

fn print_vector(x: Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
