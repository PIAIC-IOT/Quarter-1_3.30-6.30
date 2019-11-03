fn main() {

    let number_array = [100,150,200,250,300];

    println!("{:?}",number_array);
    println!("{}",number_array[1]);
    println!("{}",number_array[3]);

    // or you can store value of each index in variable and then can print that variable. 

    let index1 = number_array[1];
    let index2 = number_array[3];

    println!("{}",index1);
    println!("{}",index2);

}
