#[derive(Debug)]
enum Vehicle{
    Cars,
    Truck,
    Bikes,
}
#[derive(Debug)]
struct License {
    name:String,
    vehicle:Vehicle,
}
fn main(){
    let cars = Vehicle::Cars;
    let bikes = Vehicle::Bikes;
    let Truck = Vehicle::Truck;

    let data_1 = License {
        name : String::from("Honda"),
        vehicle: Vehicle::Cars, 
    };
    println!("{:?}" , data_1);

}