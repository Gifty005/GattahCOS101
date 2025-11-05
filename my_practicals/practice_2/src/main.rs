use std::io;

fn main() {
    let mut input = String::new();

    println!("enter height in centimeters");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let height:f32 = input.trim().parse().expect("not a valid number");

    if height >=170.0 && height <=150.0{
        println!("you are tall");
    }else if height >=144.0 && height <=130.0{
        println!("you are of average height");
    }else if height >=120.0{
        println!("you are short");
    }else{
        println!("you have abnormal height");
    }

    
}
