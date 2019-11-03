fn main(){
    let triangle = Triangle{
        length1: 25,
        length2: 80,
        length3: 60
    };


    println!("Sum : {}",triangle.sum());
    println!("Largest Number : {}",triangle.largest_size());

}

impl Triangle {
    fn sum(&self) -> u32{
        self.length1 + self.length2 + self.length3
    }

    fn largest_size(&self) -> u32{

       let mut rs = 0; 
       if self.length1 >= self.length2 && self.length1 >= self.length3{
            rs = self.length1
       } 
    
        if self.length2 >= self.length1 && self.length2 >= self.length3 {
            rs = self.length2
        }
            
        if self.length3 >= self.length1 && self.length3 >= self.length2 {
            rs = self.length3
        }

        rs

    }
}

#[derive(Debug)]
struct Triangle {
    length1: u32,
    length2: u32,
    length3: u32
}
