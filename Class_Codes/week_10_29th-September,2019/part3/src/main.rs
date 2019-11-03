fn main() {
    
    let msg1 = Message::Quit;
    let msg2 = Message::Write(String::from("hello"));
    let msg3 = Message::Move{
        x: 50,
        y: 60
    };
    let msg4 = Message::ChangeColor(10,20,30);

    // println!("{:?}",msg1);
    // println!("{:?}",msg2);
    // println!("{:?}",msg3);
    // println!("{:?}",msg4);

    msg3.call();

}

#[derive(Debug)]
enum Message{
    Quit,
    Write(String),
    Move{x: i32, y: i32},
    ChangeColor(u32,u32,u32)
}

impl Message{

    fn call(&self){
        println!("{:?}",self);
    }
}