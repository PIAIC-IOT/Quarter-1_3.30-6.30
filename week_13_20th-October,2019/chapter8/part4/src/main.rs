use std::collections::HashMap;

fn main() {
    
let teams_name = vec![String::from("KK"), String::from("LQ"), String::from("IU"),String::from("PZ")];

let teams_points = vec![10,20,30,50];

let points_table: HashMap<_,_> = teams_name.iter().zip(teams_points.iter()).collect();

println!("{:?}",points_table);

let key = String::from("KK");

let result = points_table.get(&key);

let result1 = result.unwrap();

println!("{:?}",result);
println!("{:?}",result1);

}