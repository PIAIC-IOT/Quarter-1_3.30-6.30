fn main() {

    let c = Coin::Penny;
    let mut value = value_in_cents(c);
    println!("{}", value);

    let x = Coin::Quarter(USState::Alaska);
    value = value_in_cents(x);
    println!("{}", value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }


}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}
