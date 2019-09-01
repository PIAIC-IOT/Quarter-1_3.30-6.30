


//////////////////////////////
/// Understanding variable scope
 fn main() 
 {
       let x = 5;
       let y = {
                   let x = 3;
                   x + 1
                };
        println!("The value of y is: {}", y);


   let z= {
        let a =5;
        let y= 10;
        println!("{}",x );
        30
    };
 }





/////////////////////
/// Simple function to Add two numbers
fn main()
        {

           add(10,30);

        }

    fn add(x:u32, y:u32)
            {
               println!("{}",x+y);

            }



/////////////////////////////////
/// Parsing integer into float and then dividing
fn main()
        {
            let x=10;
            let y= 4;
            let result= (x as f32) /(4 as f32);
            println!("{}",result );
        }



/////////////////////////////////
/// Accessing array and tuple in rust
fn main()
        {
            let a =(1,3,5.6);
            println!("first num of tuple is {} 2nd num of tuple is {}",a.0,a.1);//accesing elements of tuple

            let arr =[1,2,3,4];
            println!("1st element{} , 2nd element {}",arr[0],arr[2]); //accesing elements of array

            let arr_ty: [i32; 5] = [1, 2, 3, 4, 5]; //initializing array with type annotation
            println!("{:?}",arr_ty);
            
            
            let b = [0;5];//creates an array of 5 element with "0" value in each
            println!("{:?}",b );

        }


///////////////////
// Code explaining chart of integers in the book
fn main() {
    let b ='\u{1F601}';
        println!("{}",b);     // It will print emoji


    let b= true;

        println!("{}",b as i32); //parse boolean as integer


    let a = 'A';
        println!("{:b}",a as i32);// parse a as i32 and then in binary using ":b"
}
    


/////////////////////////////////////
///    Program for checking attendance using decision control statements     

fn main()
    {
        let  attendance =true;

    if attendance==true
    {
        println!("present");
    }
else 
    {
        println!("Absent");
    }

    }

