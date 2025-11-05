use std::io;

fn main() {
    
    let mut experience = String::new();
    let mut age = String::new();
    println!("is the employee experienced? (yes or no): ");
    io::stdin().read_line(&mut experience).expect("failed to read input");
    let experience=experience.trim().trim_matches(&['\r' , '\n'][..]).to_lowercase();

    println!("enter age of the employee: ");
    io::stdin().read_line(&mut age).expect("failed to read input");
    let age: u32 =age.trim().trim_matches(&['\r' , '\n'][..]).parse().expect("please enter a valid number");

    //determine incentive
    let mut _incentive:i32=0;

    if experience=="yes"&& age >= 40 {
      let _incentive = 1_560_000;
    }else if experience=="yes"&& age >=30 && age <40 {let _incentive = 1_480_000; 
    }
    else if experience=="yes"&& age <28 {
   let _incentive = 1_300_000;
    }
    else{
   let _incentive = 100_000;
    }
    println!("The annual incentive of the employee is {}", _incentive);
    
}
