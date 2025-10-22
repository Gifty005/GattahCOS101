// rust program to count numbers

use std::io;

fn main() {
   println!("Enter lower bound"); 
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("failed to read input1");
   let lower_bound:i32 = input1.trim().parse().expect("failed to input1");

   println!("Enter upper bound");
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("failed to read input2");
   let upper_bound:i32 = input2.trim().parse().expect("failed to input2");

   for x in lower_bound..upper_bound{// upper_bound is not inclusive
    println!("count level is {}",x );}
}
