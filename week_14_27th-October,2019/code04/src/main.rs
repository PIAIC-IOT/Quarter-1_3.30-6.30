use std::collections::HashMap;

fn main() {




    print_n(String::from("ukasha"),6);

    ukasha
    ukasha
    ukasha

    1
    5
    6
    10
    0









    let mut points_table = HashMap::new();

    points_table.insert(String::from("KK"), 10);
    points_table.insert(String::from("LQ"), 6);
    points_table.insert(String::from("IU"), 16);

    println!("{:#?}", points_table);

    let key = String::from("IU");

    let result = points_table.get(&key);

    println!("{:?}",result);


}
