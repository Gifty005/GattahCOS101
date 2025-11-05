use std::io;
fn main() {
    println!("Code |  Book title         | Price (naira)");
    println!("R    |Rust for beginners   | 15000.0  ");
    println!("A   |AI basics             | 12500.0  ");
    println!("D   |Data structure in rust| 20000.0  ");
    println!("N   |Networking essentials | 10000.0  ");
     //matching code
   // R >=15000;
   // A>=12500;
    // D >=20000;
    //  N>=10000;

    let mut code = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

     println!("enter book code");
    io::stdin().read_line(&mut code).expect("not a valid input"); 
   
       println!("enter quantity");
    io::stdin().read_line(&mut input2).expect("not a valid input");
    let quantity:f32=input2.trim().parse().expect("not a valid input");

     println!("price");
    io::stdin().read_line(&mut input3).expect("not a valid input");
    let price:f32=input3.trim().parse().expect("not a valid input");
    
      println!("discount in percentage");
    io::stdin().read_line(&mut input4).expect("not a valid input");
    let discount:f32=input4.trim().parse().expect("not a valid input");
    //calculating total amount
    let total_amount=price* quantity;
    let final_amount=total_amount * (discount/100.0);

    println!("final_amount is {}",final_amount );

}

