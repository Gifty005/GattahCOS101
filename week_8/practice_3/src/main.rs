// method to print to get the value
fn value(n:Option<&char>){
    println!("Element of vector {:?}",n );
}

fn main() {
    let v = vec!['R','U','S','T','A','C','I','A','N'];

    let mut input1 = String::new();
    println!("\n Enter an index value btw (0 - 8)");
    std::io::stdin().read_line(&mut input1).expect("failed to read input");

    let index:usize = input1.trim().parse().expect("failed to read input");

    //getting value at given index value
    let ch: Option<&char> = v.get(index);
    value(ch);
}
