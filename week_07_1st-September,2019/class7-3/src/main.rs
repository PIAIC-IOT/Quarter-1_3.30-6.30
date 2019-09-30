////////////////////
// this code will generate error because we are creating a dangling refrence

fn main() {

    let reference_to_nothing = dangle();// calling function and storing its return value in variable

    println!("{}",reference_to_nothing);// error will generate because s is no longer valid here

}

fn dangle() -> String {

    let s = String::from("hello");

    s // s will be drop as function end 
}
