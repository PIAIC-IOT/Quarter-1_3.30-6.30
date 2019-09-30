use std::io;
fn main() {
    println!("Please Enter a number: ");

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to read line");

    let a: i32 = a.trim().parse().unwrap(); // trim is for removing empty spaces
                                            // parse to parse string input to number
                                            // unwrap will trigger panic if the input
                                            // you are trying to parse is not a number
    
    // if you are wondering how does parse() function identify in which data type
    // we actually wanted to convert a value, in our case we wanted to convert the
    // value into i32 but we didn't told parse function about i32
    // lets observe: 

    // let a: i32 = a.trim().parse::<i32>().unwrap(); // 
    //        ^^^ this       and  ^^^^^^^ 
            
    // this is how parse function identify in which type we wanted to convert a value
    // we can remove one of these annotation but I would recommend specify in both places 
    // now if parse function find any other input then number it will unwrap the error
    // trigger panic, if you remove unwrap() it will not panic while parsing
    // we will learn more advance techniques error handling unwrap, ok and expect in chapter 9  
                                                
    println!("number is : {}", a + 6);
}
