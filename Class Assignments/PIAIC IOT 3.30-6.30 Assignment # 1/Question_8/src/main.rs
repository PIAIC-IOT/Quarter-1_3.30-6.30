fn main() {
    
    let return_value = multiplication_function(5.6,2.4,10.2);

    println!("{}",return_value);
}

fn multiplication_function(a:f64, b:f64, c:f64)->f64{

    let result = a*b*c;

    result

}

// // function can also be implemented like this

// fn multiplication_function(a:f64, b:f64, c:f64)->f64{

//     a*b*c

// }
