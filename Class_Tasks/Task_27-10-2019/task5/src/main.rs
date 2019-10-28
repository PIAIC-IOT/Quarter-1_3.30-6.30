// TASK BY DANYAL AHMED


use std::io;


fn main() {

let mut c:u32 = 0;

loop{
    let mut d = String::new();
    io::stdin().read_line(&mut d);
    let mut x:u32 = d.trim().parse().unwrap();
    if(x == 0){
        break;
    }
    else{
        c = c + x;
    }
}


    println!("The sum is: {}",c );
    
}
