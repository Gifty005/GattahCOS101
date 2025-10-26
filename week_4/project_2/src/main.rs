use std::io;

fn main() {
    
    let mut experience = String::new();
    let mut age_input = String::new();
    println!("is the employee experienced? 
        (yes or no): ");
    io::stdin().read_line(&mut experience).expect("failed to read input");
    let experience=experience.trim().trim_matches(&['\r' , '\n'][..]).to_lowercase();

    println!("enter age of the employee: ");
    io::stdin().read_line(&mut age_input).expect("failed to read input");
    let _age: u32 =age_input.trim().trim_matches(&['\r' , '\n'][..]).parse().expect("please enter a valid number");

    //determine incentive
     let age: u32 = age_input.parse().unwrap_or(0);
    let mut _incentive:i32 = 0;
    {

    if experience=="yes"{
    if age >= 40 {
      let _incentive = 1_560_000;
    }else if age >=30 && age <40 {let _incentive = 1_480_000; 
    }
    else if age <28 {
   let _incentive = 1_300_000;
    }
    else if experience == "no"{
   let _incentive = 100_000;
    }else {println!("invalid input for experience.please type 'yes' or 'no'." );

return;
}
}
    println!("The annual incentive of the employee is {}", _incentive);
    
    }
}
