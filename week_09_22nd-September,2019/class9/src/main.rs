///////////////////
///   Implementing method using struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn make_rect(x: u32, y: u32)-> Rectangle{
        Rectangle{
            width: x,
            height: y
        }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    let rect2 = Rectangle::make_rect(50, 60);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    println!("{:?}",rect2);
}
