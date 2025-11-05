use std::io;
fn main() {
let mut input1=String::new();
let mut input2=String::new();
let mut input3=String::new();

println!("enter the first side of the triangle");
io::stdin().read_line(&mut input1).expect("it is not a valid number");
let a:f32=input1.trim().parse().expect("it is not a valid number");

println!("enter the second side of the triangle");
io::stdin().read_line(&mut input2).expect ("not a valid number");
let b:f32=input2.trim().parse().expect("it is not a valid number");

println!("enter the third side of the triangle");
io::stdin().read_line(&mut input3).expect("not a valid number");
let c:f32=input3.trim().parse().expect("not a valid number");


let s:f32 = (a + b + c)/2.0;
println!("sum is {}",s );
let area:f32 = s* (s - a) * (s - b)* (s -c);
println!("area is {}", area);
}