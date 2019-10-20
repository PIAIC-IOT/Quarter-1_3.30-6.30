use std::collections::HashMap;

fn main() {

    let mut number_of_words = HashMap::new();

    let text = "hello world world wonderful world";

    for word in text.split_whitespace(){

        let count = number_of_words.entry(word).or_insert(0);

        *count +=1 ;

    }

    println!("{:?}",number_of_words);
}
