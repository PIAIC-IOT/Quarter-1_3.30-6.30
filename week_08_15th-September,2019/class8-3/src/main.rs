////////////////////////
//// making a struct name user and filling its fields using function  

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

let user1 = build_user(String::from("uk@yaho"),String::from("ukasha"));// calling function

    println!("{:#?}",user1);
}

fn build_user(email: String, username: String) -> User { //defining function with two parameters and return type is itself a struct
    
    User {          // creating instance of struct using function not variables which we were doing before
        email:email,
        username:username,
        active: true,
        sign_in_count: 1,
    }
}
