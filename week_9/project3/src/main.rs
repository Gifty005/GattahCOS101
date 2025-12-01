/*# [derive(Debug)]

struct Minister {
    name: String,
    ministry: String,
    zone:String,
}*/

fn main(){
    let commissioners = vec![
    "Aigbogun Alamba Dauda".to_string(),
    "Murtala Afeez Bendu".to_string(),
    "Okorocha Calistus Ogbona".to_string(),
    "Adewale Jimoh Akanbi".to_string(),
    "Osazuwa Faith Etieye".to_string(),
    ];

    let ministries = vec![
    "Internal Affairs".to_string(),
    "Justice".to_string(),
    "Defense".to_string(),
    "Power & Steel".to_string(),
    "Petroleum".to_string(),
    ];

    let zones = vec![
    "South West".to_string(),
    "North East".to_string(),
    "South South".to_string(),
    "South East".to_string(),
    "North West".to_string(),
    ];
    println!("merged_data\n");
    
    for i in 0..commissioners.len(){
        println!(
            "{index} {comm} {min} {zn}" , 
        index = i + 1,
        comm = commissioners[i],
        min = ministries[i],
        zn = zones[i] );
    }
}
