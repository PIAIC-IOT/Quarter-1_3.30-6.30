fn main() {
    
    add_function(10,20,30);

}

// //function that takes 3 integer arguments/parameters (x,y and z)

fn add_function(x:i32, y:i32, z:i32){
    
    let result = x+y+z; //adding all parameters in variable result

    println!("{}",result); //printing result variable
}



// // function can also be implemented like this

// fn add_function(x:i32, y:i32, z:i32){
    
//     println!("{}",x+y+z);

// }
