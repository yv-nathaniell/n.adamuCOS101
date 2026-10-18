use std::io::Write;

fn main() {
    // We are storing our string data in variables first.
    // '\n' is a newline character, so the next thing printed starts on a new line.
    let announce = "Week 9 Rust File Input & Output\n";
    let dept = "Department of Computer Science";
    
    // We create a new file called 'data.txt'.
    // If it fails (e.g., due to permissions), 'expect' will crash the program and print "create failed".
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    
    // write_all() only takes raw bytes, so we convert our string using .as_bytes() before writing.
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    
    // Let the user know the file was written successfully.
    println!("\nData written to file.");
}
