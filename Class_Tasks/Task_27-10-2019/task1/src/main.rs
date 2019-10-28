fn main(){

// function 2 arguments string and int 
// string n times print

// Question 1

times(String::from("Dan"), 3);

fn times(v: String ,n: u32){
    let mut u = 0;
    while(u < n){
        println!("{}",v );
        u = u+1;
    }
}

}