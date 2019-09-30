//////////////////
// creating struct name rectangle and calculating its area using function

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let rect1 = Rectangle //creating instance
    { 
        width: 30,
        height: 50
    };
    let rect2 = Rectangle { width: 40 , height: 60 };// creating another instance 
    
    println!("{}", area(&rect1));// calling area function and passing argument of Immutable refrence of struct
}

fn area(rectangle: &Rectangle) -> u32 { // function definition having parameter which contain Immutable refrence of struct and its return tyoe integer

    rectangle.width * rectangle.height
}
