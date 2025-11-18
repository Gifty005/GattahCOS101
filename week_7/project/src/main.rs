use std::io;
use std::f64::consts::PI;

    fn read_value_from_stdin(prompt:&str) -> f64 {
    println!("{}",prompt );
    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("failed to read line");
    shape.trim().parse().expect("please enter a shape.")
   }
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
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = choice.trim().parse().expect("Invalid choice");
       
    match choice {
        1=>calculate_trapezium_area(),
        2=>calculate_rhombus_area(),
        3=>calculate_parallelogram_area(),
        4=>calculate_cube_area(),
        5=>calculate_cylinder_volume(),
        6=>{
            println!("Exiting the program.Goodbye!");
            break;
        }
        _=> println!("Invalid choice.please enter a number between 1 and 6."),
        }

}

}

fn calculate_trapezium_area(){
    let height = read_value_from_stdin("Enter the height of the Trapezium:");
    let base1 = read_value_from_stdin("Enter the length of base1:");

    let base2 = read_value_from_stdin("Enter the length of base2:");

    let area = height / 2.0 * (base1 + base2);
    println!("The area of the Trapezium is {}",area );
}
fn calculate_rhombus_area(){
    let diagonal1 = read_value_from_stdin("Enter the lenght of diagonal1: ");
 
 let diagonal2 = read_value_from_stdin("Enter the lenght of diagonal2: ");

 let area = 0.5 * diagonal1 *diagonal2;
 println!("The area of a Rhombus is {} ",area );
}

fn calculate_parallelogram_area(){
    let base = read_value_from_stdin("please enter the base of the Parallelogram: ");

    let altitude = read_value_from_stdin("Enter the altitude of a parallellogram: ");

    let area = base * altitude;
    println!("area of the parallellogram is {} ",area );
}
fn calculate_cube_area(){
    let length = read_value_from_stdin("Enter the length of one side");
    let area = 6.0 * length.powi(2);
    println!("surface area of the cube is {} ",area );
}
fn calculate_cylinder_volume (){
    let radius = read_value_from_stdin("Enter the radius of the cylinder: ");


 let height = read_value_from_stdin("Enter the height of the cylinder: ", );

 let volume = PI * radius .powi(2) * height ;
 println!("Volume of the cylinder is {} ",volume );
}

 //println!("\nDo you want to calculate again? (yes/no):");
 //let mut again = String::new();
 //io::stdin().read_line(&mut again).expect("Failed to read");
 //again.trim().to_lowercase() != "yes" {
    // break;
//}

            
