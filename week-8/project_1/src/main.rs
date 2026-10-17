use std::io;

fn main() {
    // Vector of tuples representing the federal government APS level database[cite: 22]
    let db = vec![
        ("Office Admin", "Intern", "APS 1-2"),
        ("Academic", "Research Assistant", "APS 3-5"),
        ("Lawyer", "Junior Associate", "APS 3-5"),
        ("Lawyer", "Associate", "APS 5-8"),
        ("Teacher", "Classroom Teacher", "APS 3-5"),
        ("Office Admin", "Senior Administrator", "APS 5-8"),
        ("Lawyer", "Senior Associate 1-2", "EL1 8-10"),
        ("Academic", "Senior Lecturer", "EL2 10-13"),
        ("Lawyer", "Partner", "SES"),
    ];

    println!("--- Public Service APS Level Checker ---");
    
    let mut role_type = String::new();
    println!("Enter Role Type (e.g., Lawyer, Academic, Office Admin): ");
    io::stdin().read_line(&mut role_type).unwrap();
    let role_type = role_type.trim();

    let mut position = String::new();
    println!("Enter Specific Position (e.g., Associate, Partner): ");
    io::stdin().read_line(&mut position).unwrap();
    let position = position.trim();

    let mut found = false;
    // Iterate through the vector to validate the staff level[cite: 22]
    for entry in &db {
        if entry.0.eq_ignore_ascii_case(role_type) && entry.1.eq_ignore_ascii_case(position) {
            println!("Validated! The APS Level for {} is: {}", position, entry.2);
            found = true;
            break;
        }
    }

    if !found {
        println!("Error: Position not found in the federal level database.");
    }
}
