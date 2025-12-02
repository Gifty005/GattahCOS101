use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Student {
    name: String,
    matric_number:String,
    department:String,
    level:u32,
}
 fn main() {
    let students = vec![
    Student{
        name: "Oluchi Mordi ".to_string(),
        matric_number:"ACC10211111".to_string(),
        department:"Accounting".to_string(),
        level: 300,
    },
    Student{
        name:"Adams Aliyu ".to_string(),
        matric_number:"ECO10110101".to_string(),
        department: "Economics".to_string(),
        level: 100,
    },
    Student{
        name:"Shania Bolade".to_string(),
        matric_number:"CSC10328828".to_string(),
        department: "Computer".to_string(),
        level: 200,
    },
    Student{
        name:"Adekunle Gold".to_string(),
        matric_number:"EEE11020202".to_string(),
        department: "Electrical".to_string(),
        level: 200,
    },
    Student{
        name:"Blanca Edemoh".to_string(),
        matric_number:"MEE10202001".to_string(),
        department:"Mechanical".to_string(),
        level: 100,
    },
    ];

    //Display students
    println!("STUDENTS DETAILS:\n");
    for s in &students {
        println!("Name: {},Matric No: {},Department: {},Level: {},",s.name,s.matric_number,s.department,s.level );
    }

//save file
let mut file =File::create(" students.txt ").expect("cannot create file");

for s in &students {
    let line = format!("\n{} {} {} {}",s.name,s.matric_number,s.department,s.level);

    file.write_all(line.as_bytes())
    .expect("unable to read file ");
 
println!("Student details saved into students.txt ");
};
}