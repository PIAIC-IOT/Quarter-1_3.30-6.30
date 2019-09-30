fn main() {
    let x = 1;
    match x {
        1  => println!("{}", "One"),
        2  => println!("{}", "Two"),
        3  => println!("{}", "Three"),
        4  => println!("{}", "Four"),
        5  => println!("{}", "Five"),
        _  => println!("{}", "Any other")
    };
}