use std::io;

fn main() {
    println!("\nStudent Information Management System");

    // input name 
    println!("\n Please enter your name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Failed tp read input");
    println!("Your name is: {}",name);

    // input age 
    println!("\nenter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 = age.trim().parse().expect("input not an interger");
    println!("Your age is: {}",age);
}
