struct Employee {
   name:String,
   company:String,
   age:u32
}
fn main() {
 //create an instance of the structure
   let emp1 = Employee {
      name:String::from("Umer"),
      company:String::from("HBL"),
      age:50
   };
   let emp2 = Employee {
      name:String::from("Umer"),
//     company:String::from("HBL"),
     company:emp1.company,
      age:50
   };

 //Changing the value in the name field of a Employee instance  
//   emp1.name =String::from("Hassan");
//To get a specific value from the struct, we can use dot notation.
   // println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
}
