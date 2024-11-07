use std::collections::HashMap;
use crate::array::Contact;

mod hello;
mod range;
mod rangeodd;
mod calculate;
mod objects;
mod create;
mod array;
mod ahash;


fn main() {
    let name = "Nazar";
    const YEAR: i32 = 2007;

    println!("Range from 15-30{:?}", range::range(15, 30));
    println!("Odd range from 15-30{:?}", rangeodd::rangeodd(15, 30));
    println!("Result of calculate{:?}",calculate::calculate());
    objects::objects();
    println!("{:?}",create::create_user("Marcus", "Roma"));
    
    let phonebook = [
        Contact { name: "Marcus".to_string(), phone: "+380384804309".to_string() },
        Contact { name: "Nazar".to_string(), phone: "+380329998343".to_string() }
    ];
    println!("{:?}", array::find_phone_by_name(Box::new(phonebook),"Marcus"));

    let mut phonebook = HashMap::new();
    phonebook.insert("Marcus", "+380384804309");
    phonebook.insert("Nazar", "+380329998343");
    println!("{:?}", ahash::find_phone_by_name(&phonebook, "Marcus"));
}
