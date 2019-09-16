fn main() {

let user1 = build_user(String::from("uk@yaho"),String::from("ukasha"));

println!("{:?}",user1);

}

fn build_user(email: String, username: String) -> User {
    
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}