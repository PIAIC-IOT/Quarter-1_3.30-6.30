// TASK BY DANYAL AHMED

fn main() {


// question 3

let s = String::from("quickbrownfoxjumpsoverthedog");
let mut c = 0;
let mut d = 0;
for i in s.chars(){
    if(i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u'){
        c = c + 1;
    }
    else{
        d = d +1;

    }
}
println!("Vowels: {}",c );
println!("Consonants: {}",d );


}
