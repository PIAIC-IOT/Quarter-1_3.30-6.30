use std::io;

fn main(){
    let mut name = String::new();
    let mut age = String::new();
    let mut country = String::new();

    println!("Enter Name : ");
    io::stdin().read_line(&mut name).expect("Error ! Inavlid name");
    
    println!("Enter Age (In Numbers): ");
    io::stdin().read_line(&mut age).expect("Error ! Inavlid age");
    
    println!("Enter Country : ");
    io::stdin().read_line(&mut country).expect("Error ! Inavlid country");


    //passes input value into struct
    let person = Person{
        name : name.trim().to_string(),
        age : age.trim().parse().expect("Error ! Age is not in number"),
        country : country.trim().to_string()
    };

    let person_arr = [person.name,person.age.to_string(),person.country];
    println!("{:#?}",person_arr)

}

#[derive(Debug)]
struct Person {
    name : String,
    age : u32,
    country : String
}
