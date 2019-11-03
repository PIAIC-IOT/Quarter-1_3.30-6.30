fn main() {
    let x = 2;
    
    let message = match x {
    2 | 4 | 6 | 8 | 10  => "Even",
    1 | 3 | 5 | 7 | 9 => "Odd",
    _      => "Not in the defined range"
    };
    println!("{}", message);
}
