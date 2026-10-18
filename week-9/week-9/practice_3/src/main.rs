// 'fs' module contains functions for interacting with the file system.
use std::fs;

fn main() {
    // remove_file asks the operating system to delete the file.
    // Immediate deletion isn't guaranteed, but it triggers the system removal process.
    fs::remove_file("data.txt").expect("could not remove file");
    
    // Confirm deletion in the terminal.
    println!("file is removed");
}
