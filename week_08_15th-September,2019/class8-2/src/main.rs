///////////////////////
/// making struct with name employee having different fields
[#derive(Debug)]
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
   let mut emp2 = Employee {
      name:String::from("Younus"),
      company:emp1.company,//using the company name of first instance
      age:40
   };

 //Changing the value in the name field of a Employee instance. To change the field first we have to make an instance mutable
  emp2.name =String::from("Hassan");
   
  println!("emp2 is : {:#?}",emp2);

   
   //To get a specific value from the struct, we can use dot notation.
   println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
}
