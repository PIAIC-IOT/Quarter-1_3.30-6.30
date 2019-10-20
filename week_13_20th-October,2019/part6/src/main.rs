use std::collections::HashMap;

fn main() {


    let mut map = HashMap::new();

    map.insert(10, String::from("a"));
    map.insert(20, String::from("b"));
    map.insert(30, String::from("c"));
    map.insert(40, String::from("d"));

    map.entry(50).or_insert(String::from("t"));

    println!("{:?}",map);

}