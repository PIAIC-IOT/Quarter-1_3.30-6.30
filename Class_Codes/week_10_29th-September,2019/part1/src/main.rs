fn main() {


let four = IPAddrKind::V4;

let six = IPAddrKind::V6;

let ip_address1 = IPAddress{
    address: String::from("127.0.0.1"),
    kind: IPAddrKind::V4
};

let ip_address2 = IPAddress{
    address: String::from("::1"),
    kind: IPAddrKind::V6
};


println!("{:?}",ip_address2);


}

#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6
}

#[derive(Debug)]
struct IPAddress{
    address: String,
    kind: IPAddrKind
}
