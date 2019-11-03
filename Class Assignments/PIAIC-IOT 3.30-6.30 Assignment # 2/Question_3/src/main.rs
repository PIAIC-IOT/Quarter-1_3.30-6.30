use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();
    
    //input 3 values
    println!("Enter First Number : ");
    io::stdin().read_line(&mut x).expect("Err");
    println!("Enter Second Number : ");
    io::stdin().read_line(&mut y).expect("Err");
    println!("Enter Third Number : ");
    io::stdin().read_line(&mut z).expect("Err");

    //parse input values into float
    let x:f32 = x.trim().parse().expect("It is not a number");
    let y:f32 = y.trim().parse().expect("It is not a number");
    let z:f32 = z.trim().parse().expect("It is not a number");

    let total:f32 = (x + y + z) / 3.0;

    println!("Average : {}",total);
}
