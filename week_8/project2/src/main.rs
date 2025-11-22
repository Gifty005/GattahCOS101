
fn main() {
    //create alist of candidates
    let candidates: Vec<(String, u32)> = vec![
    ("Alice".to_string(), 5), //
    ("Gift".to_string(), 8),
    ("Moyo".to_string(), 3),
    ("Ezekiel".to_string(), 8),  //same max to showtie handling
    ];
    //find the candidate with the highest experience
    // 'max_by_key returns the first element with the greatest key.
    let best = candidates
    .iter()
    .max_by_key(|(_, years)| *years)
    .expect("The candidates list is empty");
    //print the top  candidate
    println!("Top candidate: {}({} years of programming experience)",
    best.0, best.1 );
}
