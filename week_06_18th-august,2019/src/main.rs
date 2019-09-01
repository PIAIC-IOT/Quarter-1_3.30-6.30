


////////////////////////////
/// Understanding if-else conditions
fn main()
{

    let winter= true;
    if winter
            {
                println!("garam kapray pehno bhai", );
            }

    else
            {
                println!("garmi k kapray pehan lo");
            }
}



/////////////////////
/// Iterating for loop over array
fn main()
{
    let arr = ["apple","banana","mango"];
    for i in arr.iter()
        {
            println!("hi {}",i);
        }
}



/////////////////////////////
/// Understanding while loop
fn main()
    {
        let mut x= 0;
            while x==0 
            {
                println!("hello");
                x=x+1
            }
    }




//////////////////////////////////
// array of tuple
fn main()
        {
            let arr = [("ali",15,5.2),("aslam",16,5.4),("ahmad",18,5.8)];
 
            for i in arr.iter()
                        {
                            println!("Name of Student {}",i.0);//print all the value at position 0 of tuple 
                        }
        }
