fn main() {
    let mut s = String::from("PAKISTAN");
    sub(&mut s);
        println!("{}",s);
}


fn sub(s: &mut String) -> &String{
    s.push_str(" ");
    s.push_str("ZINDABAD");
    s
}
