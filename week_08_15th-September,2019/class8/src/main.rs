fn main() {
    
    let user1 = ("Ukasha", "ukasha@yahoo.com",12345);

    let user2 = ("abc", "abc@yahoo.com",25874);

    println!("{}",user1.0);

    let black = Color(0, 0, 0);

    println!("{}",black.1);

}

struct Color(i32, i32, i32);
