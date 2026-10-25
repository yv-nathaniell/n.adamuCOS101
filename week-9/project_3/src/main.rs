use std::io::Write;

fn main() {
    let commissioners: [&str; 5]=["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries: [&str; 5] =["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones: [&str; 5] =["South West", "North East", "South South", "South West", "South East"];
    let mut combo: Vec<String>= Vec::new();

    for i in 0..commissioners.len() {
        let line= format!("{},{},{}", commissioners[i], ministries[i], zones[i]);
        combo.push(line);
    }
    let mut file= std::fs::File::create("convicministers.csv").expect("create failed");
    let fields="Commissioner,Ministry,Zone";
    println!("\n{}", fields);
    file.write_all(fields.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..combo.len() {
        println!("{}", combo[i]);
        file.write_all(combo[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nThe file has been saved as convicministers.csv");
}