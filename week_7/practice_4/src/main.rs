use std::io;
 fn main() {
    

fn add(a: i32, b:i32){
    let sum = a + b;

    println!("Sum of A and B = {}",sum );


    let mut input1 = String::new();
    println!("Enter parameter A:");
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let _a:i32 = input1.trim().parse().expect("invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter B");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let _b:i32 = input2.trim().parse().expect("invalid input");

    //call add function with arguments
   fn add(_a:i32, _b:i32){
    println!();
   }

}
}