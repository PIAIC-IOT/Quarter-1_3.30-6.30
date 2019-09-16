fn main() {

let user1 = User{
    name: String::from("ukasha"),
    email: String::from("ukasha@yahoo.com"),
    phone_no: 5222255,
    city: String::from("Karachi"),
    online_status: true,
};

let user2 = User{
    name: String::from("jawwad"),
    email: String::from("jawwad@yahoo.com"),
    phone_no: 5565656,
    ..user1
};

let user3 = User{
    name: String::from("kashif"),
    email: String::from("kashif@yahoo.com"),
    phone_no: 123456,
    city: String::from("Islamabad"),
    online_status: false,
};

println!("{:#?}",user2);

}

#[derive(Debug)]
struct User{
    name: String,
    email: String,
    phone_no: u32,
    city: String,
    online_status: bool,
}
