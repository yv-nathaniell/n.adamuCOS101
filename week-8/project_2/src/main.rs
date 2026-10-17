fn main() {
    // A vector of tuples fits this well: (name, years)[cite: 23]
    let candidates = vec![
        ("Alice", 3),
        ("Bob", 8),
        ("Charlie", 14),
        ("Diana", 6),
        ("Eve", 12),
    ];

    let mut max_years = 0;
    let mut top_developer = "";

    // Walk it with a loop, keep the highest seen so far[cite: 23]
    for developer in &candidates {
        let (name, years) = developer; // Unpack the tuple
        if *years > max_years {
            max_years = *years;
            top_developer = name;
        }
    }

    println!("--- EY Nigeria Developer Scouting ---");
    println!("The candidate with the highest programming experience is {} with {} years.", top_developer, max_years);
}
