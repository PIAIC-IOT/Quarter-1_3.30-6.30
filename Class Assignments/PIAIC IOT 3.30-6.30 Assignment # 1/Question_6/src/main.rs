fn main() {

    let my_tuple = ("IoT", "AI", "Cloud", 500.65, 8645, 65.4);

    println!("{:?}",my_tuple);
    println!("{}",my_tuple.2);
    println!("{}",my_tuple.4);
    println!("{}",my_tuple.5);

    // or you can store value of each index in variable and then can print that variable. 

    let index1 = my_tuple.2;
    let index2 = my_tuple.4;
    let index3 = my_tuple.5;

    println!("{}",index1);
    println!("{}",index2);
    println!("{}",index3);


}
