use std::{collections::HashMap, hash::Hash};


fn main () {
    let mut person:HashMap<&str, i32> = HashMap::new();

    person.insert("John", 42);
    person.insert("Bob", 28);
    person.insert("Marie", 27);

    println!("The age is {:?} ", person.get("Marie").unwrap());

    if person.contains_key("John") {
        println!("The value exists");
    } else {
        println!("The value does not exist");
    }

    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }

}