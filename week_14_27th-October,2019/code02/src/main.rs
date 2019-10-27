
use std::io;

fn main() {


    let mut buffer = Vec::new();

    for i in 0..5{

    println!("Please input your number.");

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("aa");

    buffer.push(number);

    }

    println!("{:?}",buffer);

}
