use std::collections::HashMap;

fn main() {

    let mut points_table = HashMap::new();

    points_table.insert(String::from("Karachi Kings"),6);

    points_table.insert(String::from("Lahore Q"), 0);

    points_table.insert(String::from("Islamabad U"), 10);

    println!("{:#?}",points_table);

}
