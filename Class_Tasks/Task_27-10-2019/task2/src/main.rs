fn main() {

// Question 2

let s = String::from("quickbrownfoxjumpsoverthedog");
let mut c = 0;
for i in s.chars(){
    if(i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u'){
        c = c + 1;
    }
    else{
        println!("{} its not vowels." , i );

    }
}
println!("Vowels: {}",c );

}

