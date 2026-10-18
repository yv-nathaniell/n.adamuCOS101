// We need the 'Read' trait to use the 'read_to_string' method.
use std::io::Read;

fn main() {
    // We open an EXISTING file. If it doesn't exist, unwrap() will panic (crash) the program.
    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    
    // We create an empty, mutable String to hold our file's contents.
    let mut contents = String::new();
    
    // We read all the bytes from the file into our String variable.
    file.read_to_string(&mut contents).unwrap();
    
    // Finally, we print out what we read.
    print!("{}", contents);
}
