fn main() {
    let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim ");
    println!("length is {}", fullname.len());
    println!();
    println!("After trim ");
    // trim() removes the empty spaces at the beginning and end
    println!("length is {}", fullname.trim().len());
}