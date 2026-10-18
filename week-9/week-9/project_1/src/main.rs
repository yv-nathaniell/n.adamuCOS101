// Use the Write trait so we can write bytes to the file.
use std::io::Write;

fn main() {
    // We store our datasets in arrays (vectors).
    // Some columns have more data than others, so their lengths vary.
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // Create a new file for the Nigerian Breweries data.
    let mut file = std::fs::File::create("breweries.txt").expect("create failed");
    
    // Write our header row.
    file.write_all("Lager           | Stout           | Non-Alcoholic  \n".as_bytes()).expect("write failed");
    file.write_all("---------------------------------------------------\n".as_bytes()).expect("write failed");

    // The longest list is 'lager' with 6 items, so we'll loop 6 times.
    let max_len = lager.len(); 
    
    for i in 0..max_len {
        // We use an 'if' check to prevent out-of-bounds panics on the shorter lists.
        let l = if i < lager.len() { lager[i] } else { "" };
        let s = if i < stout.len() { stout[i] } else { "" };
        let n = if i < non_alcoholic.len() { non_alcoholic[i] } else { "" };
        
        // Format the strings with padding so they align perfectly.
        // {:<15} means "pad with spaces on the right to make it exactly 15 characters wide".
        let row = format!("{:<15} | {:<15} | {:<15}\n", l, s, n);
        
        file.write_all(row.as_bytes()).expect("write failed");
    }
    
    println!("File 'breweries.txt' created successfully.");
}
