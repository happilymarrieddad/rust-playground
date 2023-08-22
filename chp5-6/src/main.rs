// Hash maps

use std::{collections::HashMap, hash::Hash};

fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();

    person.insert("Nick", 39);
    person.insert("John", 36);

    println!("The age is {:?}", person.get("Nick").unwrap());

    if person.contains_key("Nick") {
        println!("Value exists");
    } else {
        println!("Value does not exist");
    }

    match person.get("Nick") {
        Some(p) => println!("Key {} exists", p),
        None => println!("Key does not exist"),
    }

    // The order is NOT ensured
    for (name, age) in &person {
        println!("Key: {} Age: {}", name, age);
    }

    let mut likes: HashMap<&str, &str> = HashMap::new();
    // likes.insert("Nick", "peaches");
    // likes.insert("Nick", "nectarines");

    // println!("The favorite fruit is {:?}", likes.get("Nick").unwrap());

    // this will update only if the value does NOT already exist
    likes.entry("Nick").or_insert("peaches");
    println!("The favorite fruit is {:?}", likes.get("Nick").unwrap());
    likes.entry("Nick").or_insert("nectarines");
    println!("The favorite fruit is {:?}", likes.get("Nick").unwrap());

    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);
}
