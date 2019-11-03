#[derive(Debug)]
enum Vehicle{
    Cars(String),
    Truck(String),
    Bikes(String),
}
fn main(){
    let cars = Vehicle::Cars(String::from("Honda"));
    let bikes = Vehicle::Bikes(String::from("Yamaha"));
    let Truck = Vehicle::Truck(String::from("CAT"));

    println!("{:?}" , cars);
    println!("{:?}" , bikes);
    println!("{:?}" , Truck);

}
