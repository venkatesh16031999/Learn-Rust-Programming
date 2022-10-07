use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(1, "One");
    map.insert(2, "Two");
    map.insert(3, "Three");
    map.insert(4, "Four");
    map.insert(5, "Five");
    map.remove(&5);

    match map.get(&4) {
        Some(res) => println!("{:?}", res),
        None => println!("Not Found")
    }

    for (key, value) in map.iter() {
        println!("Key: {:?}, Value: {:?}", key, value);
    }

    for key in map.keys() {
        println!("Key: {:?}", key);
    }

    for value in map.values() {
        println!("Value: {:?}", value);
    }
}