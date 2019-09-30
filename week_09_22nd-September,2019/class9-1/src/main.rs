//////////////
// Using struct to find area of rectangle
    let x = Rectangle{
        width: 50,
        height: 100
    };

    println!("area of rectangle is : {:?}",x.height*x.width);

}

#[derive(Debug)]
struct Rectangle{

    width: u32,
    height: u32
}



///////////////////
/// Guessing game

use std::io;

fn main() {
    println!("Guess the number!");



    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let myint = guess.trim().parse::<i32>().unwrap();

    println!("after convert: {}", myint);

}














    // println!("Please Enter 1st number");
    // let mut a =String::new();
    // io::stdin().read_line(&mut a);
    // let mut a:i32=a.trim().parse().unwrap(); 
    // println!("1st number is {}",a+4)

    // 2
