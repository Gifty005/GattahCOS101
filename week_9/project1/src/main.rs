use std::io;
use std::fs::File;
use std::io::Write;

fn main() -> io::Result <()> {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend","Turbo king","Williams"];
    let non_acoholic=vec!["Maltina","Amstel malt", "Malta Gold", "Fayrouz"];

let mut file = File::create("drinks.txt")?;
let _ =writeln!(file, "Lager:");
for drinks in &lager{
   let _ = writeln!(file, "- {}",drinks );
};
 let _ =writeln!(file ,"\nStout:");
for drinks in &stout{
   let _ = writeln!(file, "- {}", drinks);
}
let _ = writeln!(file ,"\nNon-acoholic:");
for drinks in &non_acoholic{
  let _ =  writeln!(file, " - {}",drinks);
}
println!("Data written to drinks.txt");

Ok (())
}