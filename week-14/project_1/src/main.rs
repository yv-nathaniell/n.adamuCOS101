use std::io;
use std::io::Read;

//administrator sees the whole database
fn administrator() {
    let mut file = std::fs::File::open("globacom_db.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

//project manager sees the project table
fn project_manager() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

//employee sees the staff table
fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

//customer sees the customer table
fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

//vendor sees the dataplan table
fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn main() {
    println!("GLOBACOM");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");
    println!("Enter your role:");

    let mut role = String::new();
    io::stdin().read_line(&mut role).expect("Failed to read input");
    let role: u32 = role.trim().parse().expect("Input not an integer");

    match role {
        1 => administrator(),
        2 => project_manager(),
        3 => employee(),
        4 => customer(),
        5 => vendor(),
        _ => println!("Invalid role"),
    }
}
