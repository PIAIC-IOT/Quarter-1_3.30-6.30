fn main(){
    let rect1 = Rectangle{
        width: 50,
        height: 100
    };

    println!("Sum : {}",rect1.sum());

}

impl Rectangle {
    fn sum(&self) -> u32{
        self.width + self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
