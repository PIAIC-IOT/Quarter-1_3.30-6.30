/////////////////////
//  CREATING A VARIABLE WHICH CONTAIN STRING FROM STRING LIBRARY/HEAP passing its mutable refrence to function

fn main() {
    
    let mut s = String::from("hello");
    
    change(&mut s);//calling function
    
    println!("s is : {}",s);  
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");// concatinating string 
}
