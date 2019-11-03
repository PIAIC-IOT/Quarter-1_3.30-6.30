#[derive(Debug)]
enum Student {
    Onsite{Name:String , Roll_num : String},
    Online{Name:String , Roll_num : String},
}

fn main() {
    let st1 = Student::Onsite{ Name:String::from("Uzair") , Roll_num:String::from("R1")};
    let st2 = Student::Online{ Name:String::from("Saeed") , Roll_num:String::from("H1")};    
}
