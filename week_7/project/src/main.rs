use std::io;
use std::io::write;
use std::f64::consts::PI;
fn main() {
    loop{
        println!("\nselect a shape to calculate its area or volume:");
        println!("1. Trapezium (Area)");
        println!("2. Rhombus (Area)");
        println!("3. Parallelogram(Area)");
        println!("4. Cube(Area)");
        println!("5.Cylinder (volume");
        println!("6. Exit");

        print!("Enter your choice (1 - 6):");
        io::stdout().flush().unwrap();//ensure the prompt is displayed
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        

