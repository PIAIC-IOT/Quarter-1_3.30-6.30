// #[derive(Debug)]
// struct days{
//     monday:(String,i32),
//     Tuesday:(x:i32,y:i32)
// }
//    enum Days_ofeeks{
//        Monday,
//        Tuesday{x:i32,y:i32},
//        Wednesday
//    }
// fn main() {
 
// let mut a = days{
//     monday:(String::from("right"),3),
//     Tuesday:2
// };
// println!("{:#?}",a);
// let mut b = Days_of_weeks::Tuesday{
//     x:4,y:5
// };
// println!("{:#?}",b);



//    let a =Days_of_weeks::Monday(String::from("Monday is bad day"));
//     let x = Rectangle{
//         width: 50,
//         height: 100
//     };

//     println!("{:?}",x.height);

// }

// #[derive(Debug)]
// struct Rectangle{

//     width: u32,
//     height: u32
// }


// use 










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
