use std::io;

fn main() {
    let mut a =  String::new();
    let mut b =  String::new();
    let mut c =  String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("failed to read input");
    let a: f64 = a.trim().parse().expect("please enter enter a valid number for a");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect ("failed to read input");
    let b: f64 = b.trim().parse().expect ("please enter a valid number for b");

    println!("Enter the value of c:");
    io::stdin().read_line (&mut c).expect ("failed to read input");
    let c: f64 = c.trim().parse().expect ("please enter a valid number for c");

    //calculate the discrimant
    let d = b * b - 4.0 * a * c;

    println!("The discriminant is: {}", d);

    // finding the kind of answers we have
    if d > 0.0 {// two real numbers and different roots
        let root1 = (-b + d.sqrt())/ (2.0 * a);
        let root2 = (-b - d.sqrt())/ (2.0 * a);

        println!("two real and different roots");
        println!("root1 ={}",root1 );
        println!("root2 ={}",root2 );
    }else 
        if d ==0.0 { // one real root
            let root = -b / (2.0 * a);
            println!("one real repeated root");
            println!("root = {}",  root);

        } else {// d is negative,complex roots
            let _real_part = -b / (2.0 * a);
            let _imaginery_part = (-d).sqrt()/ (2.0 * a);
            println!("no real roots,complex roots instead");

        }

}
