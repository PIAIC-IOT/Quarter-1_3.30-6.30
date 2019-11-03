// use std::collections::HashMap;

// fn main() {


//     let mut map = HashMap::new();

//     map.insert(String::from("Blue"), 10);

//     map.insert(String::from("Red"), 60);

//     map.insert(String::from("Green"), 30);

//     // println!("{:#?}", map);

//     let v1 = vec![String::from("A"), String::from("B"), String::from("C")];

//     let v2 = vec![10,50];

//     let map_v: HashMap<_,_> = v1.iter().zip(v2.iter()).collect();

//     println!("{:#?}",map_v);


#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow"));

println!("{:?}", scores);
}

let x = ;


// }
