fn main(){
    let mut rect1 = Rectangle{
        width: 50,
        height: 100
    };
    rect1.width = 150;
    println!("{:?}",rect1);

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
