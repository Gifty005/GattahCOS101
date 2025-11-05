use std::io;
fn main() {
    //Display the menu

 println!(" 
  Menu                          price
 p=Pounded yam/Edinkaiko soup   _N3,200;
 F=Fried rice & chicken         _N3000;
 A= Amala & Ewedu soup          _N2500;
 E= Eba Egusi soup              _N2000;
 W= White rice & Stew           _N2500;");

 //Get type of food and quantity

 println!("Enter type of food(P,F,A,E.W):");
 let mut type_of_food = String::new();
 io::stdin().read_line(&mut type_of_food).expect("failed to read line");
 let type_of_food = type_of_food.trim().to_uppercase();


 println!("Enter quantity:");
 let mut quantity = String::new();
 io::stdin().read_line(&mut quantity).expect("failed to read line");
 let quantity:i32 = quantity.trim().parse().expect("please enter a number");

 //calculate the total cost
 let price = match type_of_food.as_str(){
    "P" =>3200.0,
    "F" =>3000.0,
    "A" =>2500.0,
    "E" =>2000.0,
    "W" =>2500.0,
    _ => {
        println!("invalid type_of_food.");
        return;
    }
 };
 let total_cost = price* quantity as f64;

 //apply discount if total cost exceeds N10,000
 let mut final_amount = total_cost;
 if total_cost > 10000.0 {
    let discount = total_cost * 0.05;
    final_amount = total_cost - discount;
    println!("5% discount applied!");
 } 

//display the final amount payable
println!("final_amount payable: N {}",final_amount );

}
