// OpenOptions gives us granular control over how a file is opened (read, write, append, etc.).
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    // We configure OpenOptions to allow appending, then open 'data.txt'.
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
    
    // We write new bytes. Because we opened in append mode, this text goes at the END of the file without destroying old data.
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    
    println!("file append success");
}
