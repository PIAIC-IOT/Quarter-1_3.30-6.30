use std::collections::HashMap;

fn main() {
    
    let key_field = 10;

    let value_field = String::from("PAKISTAN");

    let mut map = HashMap::new();

    map.insert(key_field, &value_field);

    println!("{:?}",map);

    println!("{}",key_field);

    println!("{}",value_field);

}
