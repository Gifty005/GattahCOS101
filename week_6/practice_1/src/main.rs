fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "pan-Atlantic University";
    let addr:&str = " km 52 Lekki- epe Expressway,Ibeju-lekki,Lagos";
    println!("Name: {}",name );
    println!("University:{}, \n Address: {},",uni,addr, );

    let department:&'static str ="computer science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {},\nschool: {},",department,school );
}
