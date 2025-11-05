use std::io;

fn main() {

    println!("condition  |  classification");
    println!("below 0c   |   freezing point");
    println!("0 - 30c    |   normal range");
    println!("above 30c  |   hot temperature");

  let mut c = String::new() ;
  
  println!("please enter temperature in celsius"); 
   io::stdin().read_line(& mut c).expect("not a valid input");
    let c:f64= c.trim().parse().expect("not a valid input");

if c == 0.0{
    println!("freezing point");
}else if c <=30.0{
    println!("normal range");
}else if c >=100.0{
    println!("hot temperature");
}else{
    println!("abnormal temperature");
}
//converting to fahrenheit

let f:f64 = (9.0/5.0)*c + 32.0;
println!("temperature in fahrenheit is {}",f );

let k:f64 = c + 273.15;
println!("temperature in kelvin is {}", k);
}
