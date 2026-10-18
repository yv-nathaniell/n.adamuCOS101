use std::io::Write;

fn main() {
    // The three datasets extracted from the cloud backup, stored as parallel arrays/vectors.
    let commissioners = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("efcc_ministers.txt").expect("create failed");
    
    let header = "Commissioner             | Ministry            | Zone\n---------------------------------------------------------------\n";
    file.write_all(header.as_bytes()).expect("write failed");
    
    // All vectors are the exact same length, so we merge by shared row index.
    for i in 0..commissioners.len() {
        // Access element 'i' from each separate vector to assemble one complete row string.
        let row = format!("{:<24} | {:<19} | {}\n", commissioners[i], ministries[i], zones[i]);
        file.write_all(row.as_bytes()).expect("write failed");
    }
    
    println!("EFCC ministers merged file 'efcc_ministers.txt' generated successfully.");
}
