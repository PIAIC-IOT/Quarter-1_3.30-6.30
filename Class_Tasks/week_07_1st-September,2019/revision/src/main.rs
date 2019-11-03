fn main() {

let mut a = String::from("hello");
    
my_function(&mut a);

println!("{}",a);

}

fn my_function(x: &mut String){

    x.push_str("world");

}
