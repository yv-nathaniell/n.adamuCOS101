use std::io::Write;

// A Struct bundles related data together into one custom type to represent a student.
struct Student {
    name: String,
    matric: String,
    dept: String,
    level: u32,
}

fn main() {
    // Create a vector (growable array) of our Student struct instances.
    let students = vec![
        Student { name: "Oluchi Mordi".to_string(), matric: "ACC10211111".to_string(), dept: "Accounting".to_string(), level: 300 },
        Student { name: "Adams Aliyu".to_string(), matric: "ECO10110101".to_string(), dept: "Economics".to_string(), level: 100 },
        Student { name: "Shania Bolade".to_string(), matric: "CSC10328828".to_string(), dept: "Computer".to_string(), level: 200 },
        Student { name: "Adekunle Gold".to_string(), matric: "EEE11020202".to_string(), dept: "Electrical".to_string(), level: 200 },
        Student { name: "Blanca Edemoh".to_string(), matric: "MEE10202001".to_string(), dept: "Mechanical".to_string(), level: 100 },
    ];

    let mut file = std::fs::File::create("students.txt").expect("create failed");
    
    // Set up our table header
    let header = "Student Name    | Matric. Number | Department  | Level\n------------------------------------------------------\n";
    
    // Step 1: Display on screen
    print!("{}", header);
    // Step 2: Write to the file
    file.write_all(header.as_bytes()).expect("write failed");

    // Loop through our vector of students
    for s in students {
        let row = format!("{:<15} | {:<14} | {:<11} | {}\n", s.name, s.matric, s.dept, s.level);
        
        // Display and Write simultaneously.
        print!("{}", row);
        file.write_all(row.as_bytes()).expect("write failed");
    }
    
    println!("\nStudent details saved to students.txt successfully!");
}
