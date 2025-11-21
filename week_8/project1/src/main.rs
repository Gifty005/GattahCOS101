use std::io;
fn main() {
    // I am storing the roles aps range pairs 
    let roles: Vec<(&str, std::ops::RangeInclusive<u32>)> = vec![
    ("office Administrator",1..=2),
    ("Intern - paralegal",3..=5),
    ("Research assistant",5..=8),
    ("Junior associate",8..=10),
    ("Classroom teacher",10..=13),
    ("Senior",     13..=u32::MAX), //"13..= 13 or higher"
    ];

    //get the level from the user
    println!("Enter an Aps level(e.g., 7):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let level: u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please enter a valid interger.");
            return;
        }
    };
    let mut found = false;
    for (role,range) in &roles{
        if range.contains (&level){
        println!("Aps level belongs to: {}{}",level ,role );
        found = true;
        break
    }
}
//if nothing matched
if !found{
    eprintln!("no role defined for Aps level.", );
}
}
