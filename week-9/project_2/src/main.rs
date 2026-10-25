use std::io::Write;

fn main() {
    let names= vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric =vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments=vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels =vec![300, 100, 200, 200, 100];
    let mut file= std::fs::File::create("studentinfo.csv").expect("create failed");

    let fields ="Student Name,Matric. Number,Department,Level";
    println!("\n{}", fields);
    file.write_all(fields.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..names.len() {
        let columns =format!("{},{},{},{}", names[i], matric[i], departments[i], levels[i]);
        println!("{}", columns);
        file.write_all(columns.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    println!("\nStudent details saved to studentinfo.csv");
}