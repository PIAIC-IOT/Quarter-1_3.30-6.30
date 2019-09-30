fn main() {
    let stud = StudentType::Onsite { 
        Name: String::from("A"), 
        RollNo: String::from("AAA-123") };
        
    match stud {
        //Destructured Struct to access values
        StudentType::Onsite {Name, RollNo} => {
                println!("Onsite Student {}", Name)
            },
        //Destructured Struct to access values
        StudentType::Online {Name, RollNo} => {
            println!("Online Student {}", RollNo)
        },
        _  => println!("{}", "Any other")
    };
}

#[derive(Debug)]
enum StudentType {
    Onsite { Name: String, RollNo: String },
    Online { Name: String, RollNo: String }
}