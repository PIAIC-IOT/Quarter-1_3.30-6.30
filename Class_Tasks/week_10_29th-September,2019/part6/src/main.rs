fn main() {
    let x = 19;
    
    match x {
        90 ..= 100  => println!("{}", "A+"),
        80 ..= 90  => println!("{}", "A"),
        70 ..= 80  => println!("{}", "B"),
        _  => println!("{}", "Failed")
    };

    //To recall range
    for i in 1..= 10 {
        println!("{}", i);
    }
}
