use std::io::Write;

fn main() {
    // putting the drinks into a vector of tuples
    let beer_list = vec![
        ("33 Export", "Legend", "Maltina"),
        ("Desperados", "Turbo King", "Amstel Malta"),
        ("Goldberg", "Williams", "Malta Gold"),
        ("Gulder", "", "Fayrouz"),
        ("Heineken", "", ""),
        ("Star", "", ""),
    ];

    let mut csv_doc = std::fs::File::create("breweries.csv").expect("system blocked the file creation tbh");
    let top_row = "Lager,Stout,Non-Alcoholic\n";
    csv_doc.write_all(top_row.as_bytes()).expect("couldnt write the top row");

    // looping through vectors one by one
    for item in &beer_list {
        let current_line = item.0.to_string() + "," + item.1 + "," + item.2 + "\n";
        csv_doc.write_all(current_line.as_bytes()).expect("Failed to save");
    }

    println!("Done, check your folder for the csv");
}