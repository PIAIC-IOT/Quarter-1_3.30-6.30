fn main() {

let ip_address = IPAddrKind::V4(String::from("127.0.0.1"));

let ip_address1 = IPAddrKind::V6(127,46.5,0);

println!("{:?}",ip_address);

println!("{:?}",ip_address1);




}

#[derive(Debug)]
enum IPAddrKind{
    V4(String),
    V6(u32,f64,u32)
}
