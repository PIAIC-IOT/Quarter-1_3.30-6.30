use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter Number : ");
    io::stdin().read_line(&mut input).expect("Err");
    let input:i32 = input.trim().parse().expect("It is not a number");

    let mut x = 0;
    while x <= input {
        let mut y = 1;
        while y <= x {
            print!("*");
            y = y + 1
        }
        println!("");
        x = x + 1;
    }

}
