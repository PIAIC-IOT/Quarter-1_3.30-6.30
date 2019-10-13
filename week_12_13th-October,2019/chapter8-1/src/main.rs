fn main() {
    

    let mut v = vec![10,20,30,40];

    for i in &mut v{

        *i += 100;

    }

    println!("{:?}",v);


}
