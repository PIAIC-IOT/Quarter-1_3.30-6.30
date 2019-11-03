fn main(){
    let some_num1= Some(150);
    let some_num2 = Some(614.98);
    let some_string1 = Some(String::from("How are you?"));
    let null_value:Option<f64> = None;
    println!("{:?}" ,some_num1);
    println!("{:?}" ,some_num2);
    println!("{:?}" ,some_string1);
    println!("{:?}" , null_value);
}
