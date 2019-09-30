////////////////////////////
//// program which shows the difference between ordinary tuple and tuple-struct

fn main() {
    
    let user1 = ("Ukasha", "ukasha@yahoo.com",12345);// tuple

    let user2 = ("abc", "abc@yahoo.com",25874);// tuple

    println!("{}",user1.0);

    let black = Color(0, 0, 0);//creating an instance of tuple struct

    println!("{}",black.1);

}

struct Color(i32, i32, i32);
