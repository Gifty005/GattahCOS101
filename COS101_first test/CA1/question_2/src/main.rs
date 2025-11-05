use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("enter name");
    io::stdin().read_line(&mut input1).expect("not a valid input");

    println!("enter hours worked");
    io::stdin().read_line(& mut input2).expect("not a valid input");
    let hours:f64= input2.trim().parse().expect("not a valid input");
 

    if hours <= 40.0{
        println!("3000.0);
    }
    else if hours>40.0{
        println!{"4500.0"}
    }
     println!("enter salary");
      io::stdin().read_line(& mut input3).expect("not a valid input");
    let salary:f64= input3.trim().parse().expect("not a valid input");

}