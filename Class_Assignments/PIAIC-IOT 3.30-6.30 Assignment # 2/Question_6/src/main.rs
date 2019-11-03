fn main() {
    let student_1 = Student{
        name : String::from("Naufil khan"),
        email : String::from("naufil_khan13@hotmail.com"),
        phone_no : String::from("+923472565746"),
        gender : Gender::Male
    };

    let student_2 = Student{
        name : String::from("Marry"),
        email : String::from("marry@gmail.com"),
        phone_no : String::from("+1223472554562"),
        gender : Gender::Female
    };

    println!("Student 1 Email : {:#?}",student_1.email);
    println!("{:#?}",student_2);
}

#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    phone_no: String,
    gender: Gender
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
