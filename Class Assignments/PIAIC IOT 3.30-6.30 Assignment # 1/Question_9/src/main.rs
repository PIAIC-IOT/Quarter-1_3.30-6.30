fn main() {
    
    let marks = 60;

    if marks >= 80{
        println!("Grade A+");
    }
    else if (marks >= 70) && (marks < 80){
        println!("Grade A");
    }
    else if (marks >= 60) && (marks < 70){
        println!("Grade B");
    }
    else if (marks >= 50) && (marks < 60){
        println!("Grade C");
    }
    else if (marks >= 40) && (marks < 50){
        println!("Grade D");
    }
    else{
        println!("Grade F");
    }
}
