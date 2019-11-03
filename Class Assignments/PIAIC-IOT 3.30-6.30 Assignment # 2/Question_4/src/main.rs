use std::io;

fn main() {
    let mut x = String::new();
    println!("Enter Your String : ");
    io::stdin().read_line(&mut x).expect("Err");
    println!("{}",cal_length(x))
}

fn cal_length(x: String) -> String{
    x.len().to_string()
}
