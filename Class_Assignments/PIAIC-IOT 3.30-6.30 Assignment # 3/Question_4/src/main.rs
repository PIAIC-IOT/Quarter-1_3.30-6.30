#[derive(Debug)]
enum Shape{
    Circle(u32),
    Triangle{a:u32,b:u32,c:u32},
    Rectengle{a:u32,b:u32,c:u32,d:u32},
    Square{a:u32,b:u32,c:u32,d:u32},
}
fn main(){
    let shape1 = Shape::Circle(8);
    let shape2 = Shape::Triangle{a:10 , b:20 , c: 30};
    let shape3 = Shape::Rectengle{a:10 , b:20 , c: 30 , d:40 };
    let shape4 = Shape::Square{a:10 , b:20 , c: 30 , d:40 };
    shape1.call();
    shape2.call();
    shape3.call();
    shape4.call();
}

impl Shape {
    fn call(&self){
        println!("Shape is {:?}", self);
    }
}
