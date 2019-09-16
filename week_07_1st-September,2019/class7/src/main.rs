fn main() {

    let s1 = String::from("hello");

    take_ownership(&s1);

    println!("{}",s1);


}

fn take_ownership(x: &String){
    
    println!("{}",x);
}


