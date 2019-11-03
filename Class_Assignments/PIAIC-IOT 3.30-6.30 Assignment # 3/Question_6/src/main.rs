#[derive(Debug)]
enum Laptop {
    HP,
    Dell(Series),
    Asus,
    Lenovo,
}
#[derive(Debug)]
enum Series{
    S_1000,
    S_2000,
    S_3000,
    S_4000,
    S_5000,
    S_6000,
}

fn main(){
    let lap1 = Laptop::Dell(Series::S_1000);
    let lap2 = Laptop::Dell(Series::S_6000);
    println!("{:#?}\n{:#?}" , lap1 , lap2);
}