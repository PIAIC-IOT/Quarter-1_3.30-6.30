fn main() {
    
    let some_number = Some(String::from("hweefs"));
    let some_string = Some(50);

    let nothing: Option<i32> = None;

    println!("{:?}",some_number);
    println!("{:?}",some_string);
    
    println!("{:?}",nothing);
}
