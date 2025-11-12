use std::io;
fn main(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("failed to read input");
    let ch:char = input.trim().parse().expect("invalid input");
    
    if ch >= '0' && ch <= '9'
    {println!("character '{}' is a digit ",ch);}
    else {
        println!("character '{}' is not a digit",ch);
    }


fn checker() {
//calling a function
println!("welcome! This program checks wether
 a character variable contains a digit or not");
checker()
}
}
