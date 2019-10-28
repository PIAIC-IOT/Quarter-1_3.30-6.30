fn main() {
// Question 4

let mut v = vec!(1,10,5,2,15);

v.sort_by(|a,b| a.cmp(b));
print!("{:?}",v );


}
